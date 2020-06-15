//! Filesystem operations

use core::{
    cell::{Cell, RefCell},
    convert::TryInto,
    fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    slice,
};

use bitflags::bitflags;
pub use heapless::consts;
use heapless::{
    pool::singleton::{Box, Pool},
    ArrayLength,
};

use crate::{
    io,
    mem::{Arena, D, F},
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
    fn lock<T>(self, f: impl FnOnce(&Inner<Self::Storage>) -> T) -> T;

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
            _inner: $crate::Private,
        }

        impl $fs {
            fn ptr() -> *mut $crate::fs::Inner<$storage> {
                use core::{cell::RefCell, mem::MaybeUninit};

                use $crate::fs::Inner;

                static mut INNER: MaybeUninit<Inner<$storage>> = MaybeUninit::uninit();

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
                    Private,
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
                    Self::ptr().write(inner);

                    // add memory to the pools before the filesystem is used
                    use $crate::mem::Pool as _; // grow_exact method

                    static mut MD: MaybeUninit<[$crate::mem::DNode; $read_dir_depth]> =
                        MaybeUninit::uninit();
                    $crate::mem::D::grow_exact(&mut MD);

                    static mut MF: MaybeUninit<[$crate::mem::FNode; $max_open_files]> =
                        MaybeUninit::uninit();
                    $crate::mem::F::grow_exact(&mut MF);

                    Ok($fs {
                        _inner: Private::new(),
                    })
                }
            }
        }

        unsafe impl $crate::fs::Filesystem for $fs {
            type Storage = $storage;

            fn lock<T>(self, f: impl FnOnce(&$crate::fs::Inner<$storage>) -> T) -> T {
                $crate::lock(|| {
                    f(unsafe { &*Self::ptr() })
                })
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
    let ret = fs.lock(|inner| {
        let mut state = inner.state.borrow_mut();
        // XXX does this (FFI call) really need a `*mut` pointer?
        unsafe { ll::lfs_fs_size(state.as_mut_ptr()) }
    });
    io::check_ret(ret)
}

/// Creates a new, empty directory at the provided path
pub fn create_dir(fs: impl Filesystem, path: &Path) -> io::Result<()> {
    fs.lock(|inner| {
        if inner.transaction_mode.get() {
            return Err(io::Error::TransactionInProgress);
        }

        let mut state = inner.state.borrow_mut();
        Ok(unsafe { ll::lfs_mkdir(state.as_mut_ptr(), path.as_ptr()) })
    })
    .and_then(|ret| io::check_ret(ret).map(drop))
}

/// Given a path, query the file system to get information about a file, directory, etc.
pub fn metadata(fs: impl Filesystem, path: &Path) -> io::Result<Metadata> {
    let mut info = MaybeUninit::uninit();
    let ret = fs.lock(|inner| {
        let mut state = inner.state.borrow_mut();
        unsafe { ll::lfs_stat(state.as_mut_ptr(), path.as_ptr(), info.as_mut_ptr()) }
    });
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
    let ret = fs.lock(|inner| unsafe {
        let mut state = inner.state.borrow_mut();
        ll::lfs_dir_open(state.as_mut_ptr(), &mut **dir, path.as_ptr())
    });
    io::check_ret(ret)?;
    Ok(ReadDir { dir, fs })
}

/// Removes a file or directory from the filesystem.
pub fn remove(fs: impl Filesystem, path: &Path) -> io::Result<()> {
    fs.lock(|inner| {
        if inner.transaction_mode.get() {
            return Err(io::Error::TransactionInProgress);
        }

        let mut state = inner.state.borrow_mut();
        Ok(unsafe { ll::lfs_remove(state.as_mut_ptr(), path.as_ptr()) })
    })
    .and_then(|ret| io::check_ret(ret).map(drop))
}

/// Rename a file or directory to a new name, replacing the original file if `to` already exists.
pub fn rename(fs: impl Filesystem, from: &Path, to: &Path) -> io::Result<()> {
    fs.lock(|inner| {
        if inner.transaction_mode.get() {
            return Err(io::Error::TransactionInProgress);
        }

        let mut state = inner.state.borrow_mut();
        Ok(unsafe { ll::lfs_rename(state.as_mut_ptr(), from.as_ptr(), to.as_ptr()) })
    })
    .and_then(|ret| io::check_ret(ret).map(drop))
}

/// Starts a filesystem transaction
///
/// Up to `N` (type level integer) files can be modified during this transaction
///
/// In this mode all writes to disk will be deferred to the `Transaction.commit` operation
///
/// # Errors
///
/// This call will error if there's at least one file currently open
///
/// While in this mode the following APIs will error:
///
/// - `fs::create_dir`
/// - `fs::remove`
/// - `fs::rename`
/// - `File::create`
/// - `File::open` -- use `Transaction::open`
/// - `File::write`, if you attempt to write more data that what can be held in the file's write
/// cache
pub fn transaction<N, F>(fs: F) -> io::Result<Transaction<F, N>>
where
    F: Filesystem,
    N: ArrayLength<File<F>>,
{
    fs.lock(|inner| {
        if inner.transaction_mode.get() {
            Err(io::Error::TransactionInProgress)
        } else if inner.open_files.get() != 0 {
            Err(io::Error::OpenFilesExist)
        } else {
            inner.storage().lock();
            inner.transaction_mode.set(true);
            Ok(Transaction {
                arena: Arena::new(),
                fs,
            })
        }
    })
}

/// A filesystem transaction
pub struct Transaction<F, N>
where
    F: Filesystem,
    N: ArrayLength<File<F>>,
{
    arena: Arena<File<F>, N>,
    fs: F,
}

impl<F, N> Transaction<F, N>
where
    F: Filesystem,
    N: ArrayLength<File<F>>,
{
    /// Opens an existing file in read/write mode
    pub fn open(&self, path: &Path) -> io::Result<&mut File<F>> {
        if self.arena.has_space() {
            let f = File::checked_open(self.fs, path, false)?;
            self.arena.alloc(f)
        } else {
            // fast path: do not try to open the file if the arena is already full
            Err(io::Error::NoMemory)
        }
    }

    /// Commits all cached writes to disk
    pub fn commit(self) -> io::Result<()> {
        self.fs.lock(|inner| {
            inner.storage().unlock();
            for f in self.arena.into_iter() {
                // FIXME don't lock again
                f.close()?;
            }
            inner.transaction_mode.set(false);
            Ok(())
        })
    }
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

        let ret = self.fs.lock(|inner| {
            let mut state = inner.state.borrow_mut();
            unsafe { ll::lfs_dir_read(state.as_mut_ptr(), &mut **self.dir, info.as_mut_ptr()) }
        });

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
        let ret = self.fs.lock(|inner| unsafe {
            let mut state = inner.state.borrow_mut();
            ll::lfs_dir_close(state.as_mut_ptr(), &mut **self.dir)
        });
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
        // NOTE this makes `state` into a self-referential struct but that's fine because it's pinned
        // in a box
        state.config.buffer = state.cache.as_mut_ptr().cast();

        fs.lock(|inner| {
            let mut fsstate = inner.state.borrow_mut();
            let ret = unsafe {
                ll::lfs_file_opencfg(
                    fsstate.as_mut_ptr(),
                    &mut state.file,
                    path.as_ptr(),
                    self.0.bits() as i32,
                    &state.config,
                )
            };
            drop(fsstate);

            io::check_ret(ret)?;
            inner.incr();
            Ok(())
        })?;

        Ok(File { fs, state })
    }
}

/// Enumeration of possible methods to seek within an I/O object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeekFrom {
    Current(i32),
    End(i32),
    Start(u32),
}

impl SeekFrom {
    fn off(self) -> i32 {
        match self {
            SeekFrom::Current(i) => i,
            SeekFrom::End(i) => i,
            // XXX handle wrap around?
            SeekFrom::Start(u) => u as i32,
        }
    }

    fn whence(self) -> u32 {
        match self {
            SeekFrom::Current(_) => ll::lfs_whence_flags_LFS_SEEK_CUR,
            SeekFrom::End(_) => ll::lfs_whence_flags_LFS_SEEK_END,
            SeekFrom::Start(_) => ll::lfs_whence_flags_LFS_SEEK_SET,
        }
    }
}

/// An open file
///
/// *NOTE* files will be automatically closed when `drop`-ped. If an I/O error occurs while closing
/// the file then the destructor will panic. To handle I/O errors that may occur when closing a file
/// use the `close` method.
pub struct File<FS>
where
    FS: Filesystem,
{
    fs: FS,
    // NOTE this must be freed only if `lfs_dir_close` was called successfully
    state: ManuallyDrop<Box<F>>,
}

// NOTE(unsafe) this is safe because `Box<F>` ("boxed FileState") owns its contents and is pinned,
// plus `FS` (handle to the filesystem) is marked as interrupt-safe (only true when "sync-cortex-a"
// is enabled)
unsafe impl<FS> Send for File<FS> where FS: Filesystem + Send {}

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
        fs.lock(|inner| {
            if inner.transaction_mode.get() {
                Err(io::Error::TransactionInProgress)
            } else {
                OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(fs, path)
            }
        })
    }

    /// Attempts to open a file in read-only mode.
    pub fn open(fs: FS, path: &Path) -> io::Result<Self> {
        Self::checked_open(fs, path, true)
    }

    fn checked_open(fs: FS, path: &Path, check: bool) -> io::Result<Self> {
        fs.lock(|inner| {
            if check && inner.transaction_mode.get() {
                Err(io::Error::TransactionInProgress)
            } else {
                // XXX it seems that the C code lets you write to files opened in read-only mode?
                OpenOptions::default().read(true).open(fs, path)
            }
        })
    }

    /// Synchronizes the file to disk and consumes this file handle, releasing resources (e.g.
    /// memory) associated to it
    pub fn close(mut self) -> io::Result<()> {
        self.close_in_place()?;
        // no need to run the destructor because we already closed the file
        mem::forget(self);
        Ok(())
    }

    /// Returns a handle to the filesystem this file lives in
    pub fn fs(&self) -> FS {
        self.fs
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    pub fn len(&mut self) -> io::Result<usize> {
        let ret = self.fs.lock(|inner| {
            let mut state = inner.state.borrow_mut();
            unsafe { ll::lfs_file_size(state.as_mut_ptr(), &mut self.state.file) }
        });
        io::check_ret(ret).map(|sz| sz as usize)
    }

    /// Reads data from the file
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ret = self.fs.lock(|inner| {
            let mut state = inner.state.borrow_mut();
            unsafe {
                ll::lfs_file_read(
                    state.as_mut_ptr(),
                    &mut self.state.file,
                    buf.as_mut_ptr().cast(),
                    buf.len().try_into().unwrap_or(u32::max_value()),
                )
            }
        });
        io::check_ret(ret).map(|sz| sz as usize)
    }

    /// Changes the position of the file
    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<usize> {
        let ret = self.fs.lock(|inner| unsafe {
            let mut state = inner.state.borrow_mut();
            ll::lfs_file_seek(
                state.as_mut_ptr(),
                &mut self.state.file,
                pos.off(),
                pos.whence() as i32,
            )
        });
        io::check_ret(ret).map(|off| off as usize)
    }

    /// Synchronizes the file to disk
    pub fn sync(&mut self) -> io::Result<()> {
        let ret = self.fs.lock(|inner| unsafe {
            ll::lfs_file_sync(inner.state.borrow_mut().as_mut_ptr(), &mut self.state.file)
        });
        io::check_ret(ret).map(drop)
    }

    /// Writes data into the file's cache
    ///
    /// To synchronize the file to disk call the `sync` method
    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let ret = self.fs.lock(|inner| unsafe {
            let mut state = inner.state.borrow_mut();
            ll::lfs_file_write(
                state.as_mut_ptr(),
                &mut self.state.file,
                data.as_ptr().cast(),
                data.len().try_into().unwrap_or(u32::max_value()),
            )
        });
        io::check_ret(ret).map(|sz| sz as usize)
    }

    fn close_in_place(&mut self) -> io::Result<()> {
        self.fs.lock(|inner| unsafe {
            let mut state = inner.state.borrow_mut();
            let ret = ll::lfs_file_close(state.as_mut_ptr(), &mut self.state.file);
            io::check_ret(ret)?;
            inner.decr();
            Ok(())
        })?;
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
    cache: [u8; crate::consts::CACHE_SIZE as usize],
    config: ll::lfs_file_config,
    file: ll::lfs_file_t,
}

#[doc(hidden)]
pub struct Inner<S>
where
    S: 'static + Storage,
{
    config: &'static Config<S>,
    state: RefCell<&'static mut State>,
    /// Number of files currently open
    open_files: Cell<usize>,
    /// Whether a `Transaction` is active
    transaction_mode: Cell<bool>,
}

#[doc(hidden)]
impl<S> Inner<S>
where
    S: Storage,
{
    pub fn new(config: &'static mut Config<S>, state: &'static mut State) -> Self {
        Self {
            state: RefCell::new(state),
            config,
            open_files: Cell::new(0),
            transaction_mode: Cell::new(false),
        }
    }

    fn decr(&self) {
        // NOTE impossible to underflow this value in safe code -- possible (and `unsafe`) if you
        // transmute a File out of thin air
        self.open_files.set(self.open_files.get() - 1)
    }

    fn incr(&self) {
        self.open_files.set(
            self.open_files
                .get()
                .checked_add(1)
                // NOTE possible in theory (`loop { forget(File::open(..)) }`) but unlikely to occur
                // in practice (due to limited amount of memory)
                .expect("file counter overflowed"),
        )
    }

    fn storage(&self) -> &S {
        unsafe { &*(self.config.inner.context as *const S) }
    }

    pub fn format(&mut self) -> io::Result<()> {
        let ret =
            unsafe { ll::lfs_format(self.state.get_mut().as_mut_ptr(), self.config.as_ptr()) };
        io::check_ret(ret).map(drop)
    }

    pub fn mount(&mut self) -> io::Result<()> {
        let ret = unsafe { ll::lfs_mount(self.state.get_mut().as_mut_ptr(), self.config.as_ptr()) };
        io::check_ret(ret).map(drop)
    }
}

#[doc(hidden)]
pub struct Buffers {
    lookahead: MaybeUninit<[u32; crate::consts::LOOKAHEADWORDS_SIZE as usize]>,
    read: MaybeUninit<[u8; crate::consts::READ_SIZE as usize]>,
    write: MaybeUninit<[u8; crate::consts::WRITE_SIZE as usize]>,
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

                attr_max: crate::consts::ATTRBYTES_MAX,
                block_count: S::BLOCK_COUNT,
                block_cycles: crate::consts::BLOCK_CYCLES,
                block_size: crate::consts::BLOCK_SIZE,
                cache_size: crate::consts::CACHE_SIZE,
                file_max: crate::consts::FILEBYTES_MAX,
                lookahead_size: 32 * crate::consts::LOOKAHEADWORDS_SIZE,
                name_max: crate::consts::FILENAME_MAX_PLUS_ONE - 1,
                prog_size: crate::consts::WRITE_SIZE,
                read_size: crate::consts::READ_SIZE,

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

    // NOTE these (C) free functions deserve some comments.
    //
    // These are basically C ABI versions of `Storage`'s methods. Because C aliasing information
    // cannot be trusted we stick to shared references (`&self`) in `Storage` methods to be on the
    // safe side.
    //
    // A more troubling issue is that we do not require `Storage` to be `Sync`. This a bit of a
    // gamble because the C library could be spawning threads and calling these `lfs_config_*`
    // functions concurrently. Ensuring soundness on this front pretty much requires reading the C
    // source code. As far as we could tell these functions are only called by the main `lfs_*` API
    // (e.g. `lfs_file_open`). This Rust wrapper (`impl Filesystem`) will only call those functions
    // after `lock`-ing the filesystem so `lfs_config_*`, themselves, do not need to be thread /
    // interrupt safe (as they'll always be called from a critical section -- on single core at
    // least)
    extern "C" fn lfs_config_read(
        config: *const ll::lfs_config,
        block: ll::lfs_block_t,
        off: ll::lfs_off_t,
        buffer: *mut cty::c_void,
        size: ll::lfs_size_t,
    ) -> cty::c_int {
        let storage = unsafe { &*((*config).context as *const S) };

        let block_size = crate::consts::BLOCK_SIZE as u32;
        let off = (block * block_size + off) as usize;
        let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize) };

        if let Err(e) = storage.read(off, buf) {
            e.into_i32()
        } else {
            0
        }
    }

    extern "C" fn lfs_config_prog(
        config: *const ll::lfs_config,
        block: ll::lfs_block_t,
        off: ll::lfs_off_t,
        buffer: *const cty::c_void,
        size: ll::lfs_size_t,
    ) -> cty::c_int {
        let storage = unsafe { &*((*config).context as *const S) };

        let block_size = crate::consts::BLOCK_SIZE as u32;
        let off = (block * block_size + off) as usize;
        let buf: &[u8] = unsafe { slice::from_raw_parts(buffer as *const u8, size as usize) };

        if let Err(e) = storage.write(off, buf) {
            e.into_i32()
        } else {
            0
        }
    }

    /// C callback interface used by LittleFS to erase data with the lower level system below the
    /// filesystem.
    extern "C" fn lfs_config_erase(
        config: *const ll::lfs_config,
        block: ll::lfs_block_t,
    ) -> cty::c_int {
        let storage = unsafe { &mut *((*config).context as *mut S) };
        let off = block as usize * crate::consts::BLOCK_SIZE as usize;

        if let Err(e) = storage.erase(off, crate::consts::BLOCK_SIZE as usize) {
            e.into_i32()
        } else {
            0
        }
    }

    /// C callback interface used by LittleFS to sync data with the lower level interface below the
    /// filesystem. Note that this function currently does nothing.
    extern "C" fn lfs_config_sync(_config: *const ll::lfs_config) -> i32 {
        // Do nothing; we presume that data is synchronized.
        0
    }
}
