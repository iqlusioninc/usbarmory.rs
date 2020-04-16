//! Filesystem operations

use core::{
    cell::RefCell,
    convert::TryInto,
    fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    slice,
};

use bitflags::bitflags;
use heapless::pool::singleton::{Box, Pool};

use crate::{
    consts, io,
    mem::{D, F},
    path::{Path, PathBuf},
    storage::Storage,
};

/// A filesystem
///
/// *NOTE* do not implement this trait yourself; use the `filesystem!` macro
///
/// # Safety
/// - Implementer must be a singleton
pub unsafe trait Filesystem: Copy {
    /// Storage device this filesystem commits changes to
    type Storage: Storage + 'static;

    #[doc(hidden)]
    fn handle(self) -> &'static RefCell<Inner<Self::Storage>>;

    /// Mounts the filesystem
    ///
    /// This consume the `Storage` device (singleton) and thus can only be called at most once
    ///
    /// The `format` flag indicates whether to format the filesystem before mounting it
    fn mount(storage: Self::Storage, format: bool) -> io::Result<Self>;
}

/// Declares a filesystem named `$fs` that uses `$Storage` as the storage device
///
/// `$Storage` must implement the `Storage` trait
///
/// This macro will (safely) implement the unsafe `Filesystem` trait
///
/// `$fs` will become a share-able handle to  a `Filesystem` singleton. Think of `$fs` as a
/// `&'static _` reference.
#[macro_export]
macro_rules! filesystem {
    (
        $(#[$attr:meta])*
        $fs:ident,
        Storage=$storage:ty,
        max_open_files=$max_open_files:expr,
        read_dir_depth=$read_dir_depth:expr
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy)]
        pub struct $fs {
            _inner: $crate::NotSendOrSync,
        }

        impl $fs {
            fn ptr() -> *mut core::cell::RefCell<$crate::fs::Inner<$storage>> {
                use core::{cell::RefCell, mem::MaybeUninit};

                use $crate::fs::Inner;

                static mut INNER: MaybeUninit<RefCell<Inner<$storage>>> = MaybeUninit::uninit();

                unsafe { INNER.as_mut_ptr() }
            }

            /// See `Filesystem.mount`
            pub fn mount(storage: $storage, format: bool) -> $crate::io::Result<Self> {
                use core::{
                    cell::RefCell,
                    mem::MaybeUninit,
                    sync::atomic::{AtomicBool, Ordering},
                };

                use $crate::{
                    fs::{Buffers, Config, Inner, State},
                    NotSendOrSync,
                };

                // NOTE(unsafe) this section is executed at most once because `storage` is an owned
                // singleton
                static mut BUFFERS: Buffers = Buffers::uninit();
                // NOTE cannot (partially) construct `Config` in `const` context
                static mut CONFIG: MaybeUninit<Config<$storage>> = MaybeUninit::uninit();
                static mut STATE: State = State::uninit();
                static mut STORAGE: MaybeUninit<$storage> = MaybeUninit::uninit();

                unsafe {
                    STORAGE.as_mut_ptr().write(storage);
                    CONFIG
                        .as_mut_ptr()
                        .write(Config::new(&mut BUFFERS, &*STORAGE.as_ptr()));
                    let mut inner = Inner::new(&mut *CONFIG.as_mut_ptr(), &mut STATE);
                    if format {
                        inner.format()?;
                    }
                    inner.mount()?;
                    Self::ptr().write(RefCell::new(inner));

                    // add memory to the pools before the filesystem is used
                    use $crate::mem::Pool as _; // grow exact method

                    static mut MD: MaybeUninit<[$crate::mem::DNode; $read_dir_depth]> =
                        MaybeUninit::uninit();
                    $crate::mem::D::grow_exact(&mut MD);

                    static mut MF: MaybeUninit<[$crate::mem::FNode; $max_open_files]> =
                        MaybeUninit::uninit();
                    $crate::mem::F::grow_exact(&mut MF);

                    Ok($fs {
                        _inner: NotSendOrSync::new(),
                    })
                }
            }
        }

        unsafe impl $crate::fs::Filesystem for $fs {
            type Storage = $storage;

            fn handle(self) -> &'static core::cell::RefCell<$crate::fs::Inner<$storage>> {
                unsafe { &*Self::ptr() }
            }

            fn mount(storage: $storage, format: bool) -> $crate::io::Result<Self> {
                Self::mount(storage, format)
            }
        }
    };
}

/// Returns the number of available blocks
///
/// *NOTE* this is an approximation of free space on the storage device
pub fn available_blocks<FS>(fs: FS) -> io::Result<u32>
where
    FS: Filesystem,
{
    Ok(FS::Storage::BLOCK_COUNT - used_blocks(fs)?)
}

fn used_blocks(fs: impl Filesystem) -> io::Result<u32> {
    let mut fs = fs.handle().borrow_mut();
    // XXX does this really need a `*mut` pointer?
    let ret = unsafe { ll::lfs_fs_size(fs.state.as_mut_ptr()) };
    drop(fs);
    io::check_ret(ret)
}

/// Creates a new, empty directory at the provided path
pub fn create_dir(fs: impl Filesystem, path: &Path) -> io::Result<()> {
    let mut fs = fs.handle().borrow_mut();
    let ret = unsafe { ll::lfs_mkdir(fs.state.as_mut_ptr(), path.as_ptr()) };
    drop(fs);
    io::check_ret(ret).map(drop)
}

/// Given a path, query the file system to get information about a file, directory, etc.
pub fn metadata(fs: impl Filesystem, path: &Path) -> io::Result<Metadata> {
    let mut f = fs.handle().borrow_mut();
    let mut info = MaybeUninit::uninit();
    let ret = unsafe { ll::lfs_stat(f.state.as_mut_ptr(), path.as_ptr(), info.as_mut_ptr()) };
    drop(f);
    io::check_ret(ret)?;
    Ok(Metadata::from_info(unsafe { info.assume_init() }))
}

/// Returns an iterator over the entries within a directory.
pub fn read_dir<FS>(fs: FS, path: &Path) -> io::Result<ReadDir<FS>>
where
    FS: Filesystem,
{
    let mut dir = ManuallyDrop::new(
        D::alloc()
            .ok_or(io::Error::NoMemory)?
            // FIXME(upstream) it should not be necessary to zero the allocation
            .init(unsafe { mem::zeroed() }),
    );
    let mut f = fs.handle().borrow_mut();
    let ret = unsafe { ll::lfs_dir_open(f.state.as_mut_ptr(), &mut **dir, path.as_ptr()) };
    drop(f);
    io::check_ret(ret)?;
    Ok(ReadDir { dir, fs })
}

/// Removes a file or directory from the filesystem.
pub fn remove(fs: impl Filesystem, path: &Path) -> io::Result<()> {
    let mut fs = fs.handle().borrow_mut();
    let ret = unsafe { ll::lfs_remove(fs.state.as_mut_ptr(), path.as_ptr()) };
    drop(fs);
    io::check_ret(ret).map(drop)
}

/// Rename a file or directory to a new name, replacing the original file if `to` already exists.
pub fn rename(fs: impl Filesystem, from: &Path, to: &Path) -> io::Result<()> {
    let mut fs = fs.handle().borrow_mut();
    let ret = unsafe { ll::lfs_rename(fs.state.as_mut_ptr(), from.as_ptr(), to.as_ptr()) };
    drop(fs);
    io::check_ret(ret).map(drop)
}

/// Iterator over the entries in a directory.
///
/// *NOTE* this value is effectively an *open* directory that must eventually be closed. Its
/// destructor will close the directory and panic if any I/O error occurred during the close
/// operation. To handle potential I/O errors call `close` on this value.
pub struct ReadDir<FS>
where
    FS: Filesystem,
{
    // NOTE this must be freed only if `lfs_dir_close` was called successfully
    dir: ManuallyDrop<Box<D>>,
    fs: FS,
}

impl<FS> Iterator for ReadDir<FS>
where
    FS: Filesystem,
{
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut info = MaybeUninit::<ll::lfs_info>::uninit();

        let mut fs = self.fs.handle().borrow_mut();
        let ret =
            unsafe { ll::lfs_dir_read(fs.state.as_mut_ptr(), &mut **self.dir, info.as_mut_ptr()) };
        drop(fs);

        if ret == 0 {
            None
        } else if let Err(e) = io::check_ret(ret) {
            Some(Err(e))
        } else {
            let info = unsafe { info.assume_init() };
            let entry = DirEntry {
                metadata: Metadata::from_info(info),
            };

            Some(Ok(entry))
        }
    }
}

impl<FS> ReadDir<FS>
where
    FS: Filesystem,
{
    /// Closes this directory, releasing resources (e.g. memory) associated to it
    pub fn close(mut self) -> io::Result<()> {
        self.close_in_place()?;
        // no need to run the destructor because we already closed the directory
        mem::forget(self);
        Ok(())
    }

    fn close_in_place(&mut self) -> io::Result<()> {
        let mut fs = self.fs.handle().borrow_mut();
        let ret = unsafe { ll::lfs_dir_close(fs.state.as_mut_ptr(), &mut **self.dir) };
        drop(fs);
        io::check_ret(ret)?;
        // now that we have unliked (self.)`dir` from (self.)`fs` we can release `dir`'s memory
        unsafe { ManuallyDrop::drop(&mut self.dir) }
        Ok(())
    }
}

impl<FS> Drop for ReadDir<FS>
where
    FS: Filesystem,
{
    fn drop(&mut self) {
        self.close_in_place()
            .expect("error while closing directory")
    }
}

/// Entry returned by the `ReadDir` iterator
pub struct DirEntry {
    metadata: Metadata,
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirEntry")
            .field("metadata", self.metadata())
            .finish()
    }
}

impl DirEntry {
    /// Returns the bare file name of this directory entry without any other leading path component.
    pub fn file_name(&self) -> &Path {
        self.metadata.file_name()
    }

    /// Returns the file type for the file that this entry points at.
    pub fn file_type(&self) -> FileType {
        self.metadata.file_type()
    }

    /// Returns the metadata for the file or directory that this entry points at.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// Metadata information about a file or directory
#[derive(Clone, Debug)]
pub struct Metadata {
    file_name: PathBuf,
    file_type: FileType,
    size: usize,
}

// NOTE(allow) `std::fs` version does not have an `is_empty` method
#[allow(clippy::len_without_is_empty)]
impl Metadata {
    fn from_info(info: ll::lfs_info) -> Self {
        Self {
            file_name: unsafe { PathBuf::from_buffer(info.name) },
            file_type: match info.type_ as ll::lfs_type {
                ll::lfs_type_LFS_TYPE_DIR => FileType::Dir,
                ll::lfs_type_LFS_TYPE_REG => FileType::File,
                _ => unreachable!(),
            },
            size: info.size as usize,
        }
    }

    /// Returns `true` if this metadata is for a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    /// Returns `true` if this metadata is for a regular file
    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    pub fn file_name(&self) -> &Path {
        &self.file_name
    }

    /// Returns the file type for this metadata.
    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    pub fn len(&self) -> usize {
        self.size
    }
}

/// A structure representing a type of file with accessors for each file type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileType {
    /// File
    File,
    /// Directory
    Dir,
}

impl FileType {
    /// Tests whether this file type represents a directory.
    pub fn is_dir(self) -> bool {
        self == FileType::Dir
    }

    /// Tests whether this file type represents a regular file
    pub fn is_file(self) -> bool {
        self == FileType::File
    }
}

bitflags! {
    struct FileOpenFlags: u32 {
        const READ = 0x1;
        const WRITE = 0x2;
        const READWRITE = Self::READ.bits | Self::WRITE.bits;
        const CREATE = 0x0100;
        const EXCL = 0x0200;
        const TRUNCATE = 0x0400;
        const APPEND = 0x0800;
    }
}

struct OpenOptions(FileOpenFlags);

impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenOptions {
    fn new() -> Self {
        OpenOptions(FileOpenFlags::empty())
    }

    fn create(&mut self, create: bool) -> &mut Self {
        if create {
            self.0.insert(FileOpenFlags::CREATE)
        } else {
            self.0.remove(FileOpenFlags::CREATE)
        }
        self
    }

    fn read(&mut self, read: bool) -> &mut Self {
        if read {
            self.0.insert(FileOpenFlags::READ)
        } else {
            self.0.remove(FileOpenFlags::READ)
        }
        self
    }

    fn truncate(&mut self, truncate: bool) -> &mut Self {
        if truncate {
            self.0.insert(FileOpenFlags::TRUNCATE)
        } else {
            self.0.remove(FileOpenFlags::TRUNCATE)
        }
        self
    }

    fn write(&mut self, write: bool) -> &mut Self {
        if write {
            self.0.insert(FileOpenFlags::WRITE)
        } else {
            self.0.remove(FileOpenFlags::WRITE)
        }
        self
    }

    fn open<FS>(&self, fs: FS, path: &Path) -> io::Result<File<FS>>
    where
        FS: Filesystem,
    {
        let mut state = ManuallyDrop::new(
            F::alloc()
                .ok_or(io::Error::NoMemory)?
                // FIXME(upstream) it should not be necessary to zero the memory block
                .init(unsafe { mem::zeroed() }),
        );
        // NOTE this makes `state` into a self-referential struct but it's fined because it's pinned
        // in a box
        state.config.buffer = state.cache.as_mut_ptr().cast();

        let mut f = fs.handle().borrow_mut();
        let ret = unsafe {
            ll::lfs_file_opencfg(
                f.state.as_mut_ptr(),
                &mut state.file,
                path.as_ptr(),
                self.0.bits() as i32,
                &state.config,
            )
        };
        drop(f);

        io::check_ret(ret)?;
        Ok(File { fs, state })
    }
}

/// An open file
pub struct File<FS>
where
    FS: Filesystem,
{
    fs: FS,
    // NOTE this must be freed only if `lfs_dir_close` was called successfully
    state: ManuallyDrop<Box<F>>,
}

// NOTE(allow) `std::fs` version does not have an `is_empty` method
#[allow(clippy::len_without_is_empty)]
impl<FS> File<FS>
where
    FS: Filesystem,
{
    /// Opens a file in write-only mode.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    pub fn create(fs: FS, path: &Path) -> io::Result<Self> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(fs, path)
    }

    /// Attempts to open a file in read-only mode.
    pub fn open(fs: FS, path: &Path) -> io::Result<Self> {
        OpenOptions::default().read(true).open(fs, path)
    }

    /// Synchronizes the file to disk and consumes this file handle, releasing resources (e.g.
    /// memory) associated to it
    pub fn close(mut self) -> io::Result<()> {
        self.close_in_place()?;
        // no need to run the destructor because we already closed the file
        mem::forget(self);
        Ok(())
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    pub fn len(&mut self) -> io::Result<usize> {
        let mut fs = self.fs.handle().borrow_mut();
        let ret = unsafe { ll::lfs_file_size(fs.state.as_mut_ptr(), &mut self.state.file) };
        drop(fs);
        io::check_ret(ret).map(|sz| sz as usize)
    }

    /// Reads data from the file
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut fs = self.fs.handle().borrow_mut();
        let ret = unsafe {
            ll::lfs_file_read(
                fs.state.as_mut_ptr(),
                &mut self.state.file,
                buf.as_mut_ptr().cast(),
                buf.len().try_into().unwrap_or(u32::max_value()),
            )
        };
        drop(fs);
        io::check_ret(ret).map(|sz| sz as usize)
    }

    /// Synchronizes the file to disk
    pub fn sync(&mut self) -> io::Result<()> {
        let mut fs = self.fs.handle().borrow_mut();
        let ret = unsafe { ll::lfs_file_sync(fs.state.as_mut_ptr(), &mut self.state.file) };
        drop(fs);
        io::check_ret(ret).map(drop)
    }

    /// Writes data into the file's cache
    ///
    /// To synchronize the file to disk call the `sync` method
    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let mut fs = self.fs.handle().borrow_mut();
        let ret = unsafe {
            ll::lfs_file_write(
                fs.state.as_mut_ptr(),
                &mut self.state.file,
                data.as_ptr().cast(),
                data.len().try_into().unwrap_or(u32::max_value()),
            )
        };
        drop(fs);
        io::check_ret(ret).map(|sz| sz as usize)
    }

    fn close_in_place(&mut self) -> io::Result<()> {
        let mut fs = self.fs.handle().borrow_mut();
        let ret = unsafe { ll::lfs_file_close(fs.state.as_mut_ptr(), &mut self.state.file) };
        drop(fs);
        io::check_ret(ret)?;
        // now that we have unliked (self.)`dir` from (self.)`fs` we can release `dir`'s memory
        unsafe { ManuallyDrop::drop(&mut self.state) }
        Ok(())
    }
}

impl<FS> Drop for File<FS>
where
    FS: Filesystem,
{
    fn drop(&mut self) {
        self.close_in_place().expect("error while closing file")
    }
}

#[doc(hidden)]
pub struct FileState {
    cache: [u8; consts::CACHE_SIZE as usize],
    config: ll::lfs_file_config,
    file: ll::lfs_file_t,
}

#[doc(hidden)]
pub struct Inner<S>
where
    S: 'static + Storage,
{
    config: &'static Config<S>,
    state: &'static mut State,
}

#[doc(hidden)]
impl<S> Inner<S>
where
    S: Storage,
{
    pub fn new(config: &'static mut Config<S>, state: &'static mut State) -> Self {
        Self { state, config }
    }

    pub fn format(&mut self) -> io::Result<()> {
        let ret = unsafe { ll::lfs_format(self.state.as_mut_ptr(), self.config.as_ptr()) };
        io::check_ret(ret).map(drop)
    }

    pub fn mount(&mut self) -> io::Result<()> {
        let ret = unsafe { ll::lfs_mount(self.state.as_mut_ptr(), self.config.as_ptr()) };
        io::check_ret(ret).map(drop)
    }
}

#[doc(hidden)]
pub struct Buffers {
    lookahead: MaybeUninit<[u32; consts::LOOKAHEADWORDS_SIZE as usize]>,
    read: MaybeUninit<[u8; consts::READ_SIZE as usize]>,
    write: MaybeUninit<[u8; consts::WRITE_SIZE as usize]>,
}

#[doc(hidden)]
impl Buffers {
    pub const fn uninit() -> Self {
        Self {
            lookahead: MaybeUninit::uninit(),
            read: MaybeUninit::uninit(),
            write: MaybeUninit::uninit(),
        }
    }
}

#[doc(hidden)]
pub struct State {
    inner: MaybeUninit<ll::lfs_t>,
}

#[doc(hidden)]
impl State {
    pub const fn uninit() -> Self {
        Self {
            inner: MaybeUninit::uninit(),
        }
    }

    fn as_mut_ptr(&mut self) -> *mut ll::lfs_t {
        self.inner.as_mut_ptr()
    }
}

#[doc(hidden)]
pub struct Config<S>
where
    S: Storage,
{
    _storage: PhantomData<S>,
    inner: ll::lfs_config,
}

#[doc(hidden)]
impl<S> Config<S>
where
    S: Storage,
{
    pub fn new(buffers: &'static mut Buffers, storage: &'static S) -> Self {
        Self {
            _storage: PhantomData,
            inner: ll::lfs_config {
                read: Some(Self::lfs_config_read),
                prog: Some(Self::lfs_config_prog),
                erase: Some(Self::lfs_config_erase),
                sync: Some(Self::lfs_config_sync),

                attr_max: consts::ATTRBYTES_MAX,
                block_count: S::BLOCK_COUNT,
                block_cycles: consts::BLOCK_CYCLES,
                block_size: consts::BLOCK_SIZE,
                cache_size: consts::CACHE_SIZE,
                file_max: consts::FILEBYTES_MAX,
                lookahead_size: 32 * consts::LOOKAHEADWORDS_SIZE,
                name_max: consts::FILENAME_MAX_PLUS_ONE - 1,
                prog_size: consts::WRITE_SIZE,
                read_size: consts::READ_SIZE,

                context: storage as *const S as *mut _,
                lookahead_buffer: buffers.lookahead.as_mut_ptr().cast(),
                read_buffer: buffers.read.as_mut_ptr().cast(),
                prog_buffer: buffers.write.as_mut_ptr().cast(),
            },
        }
    }

    fn as_ptr(&self) -> *const ll::lfs_config {
        &self.inner
    }

    extern "C" fn lfs_config_read(
        config: *const ll::lfs_config,
        block: ll::lfs_block_t,
        off: ll::lfs_off_t,
        buffer: *mut cty::c_void,
        size: ll::lfs_size_t,
    ) -> cty::c_int {
        let storage = unsafe { &*((*config).context as *const S) };

        let block_size = consts::BLOCK_SIZE as u32;
        let off = (block * block_size + off) as usize;
        let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize) };

        // TODO error handling?
        storage.read(off, buf).unwrap();
        0
    }

    extern "C" fn lfs_config_prog(
        config: *const ll::lfs_config,
        block: ll::lfs_block_t,
        off: ll::lfs_off_t,
        buffer: *const cty::c_void,
        size: ll::lfs_size_t,
    ) -> cty::c_int {
        let storage = unsafe { &*((*config).context as *const S) };

        let block_size = consts::BLOCK_SIZE as u32;
        let off = (block * block_size + off) as usize;
        let buf: &[u8] = unsafe { slice::from_raw_parts(buffer as *const u8, size as usize) };

        // TODO error handling?
        storage.write(off, buf).unwrap();
        0
    }

    /// C callback interface used by LittleFS to erase data with the lower level system below the
    /// filesystem.
    extern "C" fn lfs_config_erase(
        config: *const ll::lfs_config,
        block: ll::lfs_block_t,
    ) -> cty::c_int {
        let storage = unsafe { &mut *((*config).context as *mut S) };
        let off = block as usize * consts::BLOCK_SIZE as usize;

        // TODO error handling?
        storage.erase(off, consts::BLOCK_SIZE as usize).unwrap();
        0
    }

    /// C callback interface used by LittleFS to sync data with the lower level interface below the
    /// filesystem. Note that this function currently does nothing.
    extern "C" fn lfs_config_sync(_config: *const ll::lfs_config) -> i32 {
        // Do nothing; we presume that data is synchronized.
        0
    }
}
