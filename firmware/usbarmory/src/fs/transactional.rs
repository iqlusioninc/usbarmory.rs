//! Transactional FS

use core::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
    ptr::NonNull,
};

use generic_array::GenericArray;
use heapless::ArrayLength;
use littlefs2::{
    fs::{self, SeekFrom},
    io::{self, Read, Seek, Write},
};

use super::{FileAlloc, LfsStorage, LittleFs};
use crate::storage::ManagedBlockDevice;

/// Transactional FS
///
/// This FS does NOT allow `fs`-style operations like `create_dir` and `remove` that require writing
/// to the eMMC. It does allow `File` write operations but those will be cached to memory and only
/// be committed to the eMMC when the `sync` method (which consumes `self`) is called
pub struct Fs<'fs, 'f, D, N>
where
    D: ManagedBlockDevice,
    N: ArrayLength<File<'f, 'fs, D>>,
{
    littlefs: LittleFs<'fs, D>,
    files: Arena<File<'f, 'fs, D>, N>,
}

/// An open file.
///
/// All writes to this file will be cached in memory until `Fs.sync` is called
pub struct File<'f, 'fs, D>
where
    D: ManagedBlockDevice,
    'fs: 'f,
{
    file: fs::File<'f, LfsStorage<D>>,

    // NOTE this field (`self.fs`) is a pointer into `Fs.littlefs` where `self` (this file) will be
    // allocated in `Fs.arena` (the other field of the *same* `Fs` instance). The `File` type will
    // only ever appear in user code as the mutable reference `&'Fs mut File` where `'Fs` is the
    // lifetime of the `Fs` instance that holds the actual `File` instance.
    //
    // Given that (a) `&'Fs mut File` can NOT outlive `Fs` because of the `'Fs` lifetime, (b) one cannot
    // move `Fs` while `&'Fs mut File` exists and (c) the `File` instance will be deallocated at the
    // same time as the `Fs` instance and , it follows that accessing this `fs` field will NOT
    // result in accessing deallocated memory. Note that other borrow checking invariants need to be
    // enforced manually (e.g. safe public API must not create references with lifetime greater than
    // `'Fs` from this raw pointer; also Rust aliasing rules need to be enforced)
    littlefs: NonNull<LittleFs<'fs, D>>,
}

impl<'f, 'fs, D> File<'f, 'fs, D>
where
    D: ManagedBlockDevice,
{
    /// Opens the file at `path`.
    pub fn open<'lfs, N>(
        fs: &'lfs Fs<'fs, 'f, D, N>,
        alloc: &'f mut FileAlloc<D>,
        path: impl AsRef<[u8]>,
    ) -> io::Result<&'lfs mut Self>
    where
        N: ArrayLength<File<'f, 'fs, D>>,
    {
        let mut lf = fs::File::open(
            path.as_ref(),
            &mut alloc.inner,
            &mut fs.littlefs.fs.borrow_mut(),
            &mut fs.littlefs.storage.borrow_mut(),
        )?;
        lf.seek(
            &mut fs.littlefs.fs.borrow_mut(),
            &mut fs.littlefs.storage.borrow_mut(),
            SeekFrom::Start(0),
        )?;
        let f = File {
            file: lf,
            littlefs: NonNull::from(&fs.littlefs),
        };
        Ok(fs.files.alloc(f)?)
    }

    /// Returns the length of this file in Bytes.
    pub fn len(&mut self) -> io::Result<usize> {
        // NOTE(unsafe) this *shared* reference (`&'_`) will only be used within the scope of this
        // function and the `write` operation is blocking
        let littlefs = unsafe { self.littlefs.as_ref() };
        self.file.len(
            &mut littlefs.fs.borrow_mut(),
            &mut littlefs.storage.borrow_mut(),
        )
    }

    /// Reads bytes from this file into `buf`.
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // NOTE(unsafe) this *shared* reference (`&'_`) will only be used within the scope of this
        // function and the `write` operation is blocking
        let littlefs = unsafe { self.littlefs.as_ref() };
        self.file.read(
            &mut littlefs.fs.borrow_mut(),
            &mut littlefs.storage.borrow_mut(),
            buf,
        )
    }

    /// Writes byte from `buf` into this file.
    ///
    /// The data will be cached in memory and only be committed to disk when `Fs.sync` is called
    pub fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        // NOTE(unsafe) this *shared* reference (`&'_`) will only be used within the scope of this
        // function and the `write` operation is blocking
        let littlefs = unsafe { self.littlefs.as_ref() };
        self.file.write(
            &mut littlefs.fs.borrow_mut(),
            &mut littlefs.storage.borrow_mut(),
            bytes,
        )
    }
}

impl<'fs, 'f, D, N> Fs<'fs, 'f, D, N>
where
    D: ManagedBlockDevice,
    N: ArrayLength<File<'f, 'fs, D>>,
{
    pub(crate) fn wrap(littlefs: LittleFs<'fs, D>) -> Self {
        Self {
            littlefs,
            files: Arena::new(),
        }
    }

    /// Returns the available space in Bytes (approximated).
    #[cfg(untested)]
    pub fn available_space(&self) -> io::Result<u64> {
        self.littlefs
            .fs
            .borrow_mut()
            .available_space(&mut self.littlefs.storage.borrow_mut())
            .map(|space| space as u64)
    }

    /// Returns an iterator over the contents of the directory at `path`.
    #[cfg(untested)]
    pub fn read_dir<'r>(&'r self, path: impl AsRef<[u8]>) -> io::Result<ReadDir<'r, 'fs, D>> {
        self.littlefs
            .fs
            .borrow_mut()
            .read_dir(path.as_ref(), &mut self.littlefs.storage.borrow_mut())
            .map(move |inner| ReadDir {
                fs: &self.littlefs,
                inner,
            })
    }

    /// Commits all pending file writes to the eMMC
    ///
    /// NOTE this will synchronize *one file at a time* and *in the order they were opened* (from
    /// first opened to last opened)
    pub fn sync(mut self) -> io::Result<LittleFs<'fs, D>> {
        // can perform eMMC writes again
        self.littlefs.storage.borrow_mut().read_only = false;
        let bufferp = self.files.buffer.get() as *mut File<'f, 'fs, D>;
        let n = self.files.pos.get();
        for i in 0..n {
            // NOTE(unsafe) OK to move out the `File` because we are consuming the `files` arena
            let f = unsafe { bufferp.add(i).read() };
            // NB `f.fs` is likely a dangling pointer by now so use `self.littlefs` instead
            f.file
                .close(self.littlefs.fs.get_mut(), self.littlefs.storage.get_mut())?;
        }
        Ok(self.littlefs)
    }
}

// a heapless Arena
// NOTE this is missing a `Drop` implementation but it doesn't matter because we'll `close` all
// files by hand (rather than relying on destructors running) in `Fs.sync`
struct Arena<T, N>
where
    N: ArrayLength<T>,
{
    buffer: UnsafeCell<MaybeUninit<GenericArray<T, N>>>,
    pos: Cell<usize>,
}

impl<T, N> Arena<T, N>
where
    N: ArrayLength<T>,
{
    fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(MaybeUninit::uninit()),
            pos: Cell::new(0),
        }
    }

    fn alloc(&self, value: T) -> io::Result<&mut T> {
        let i = self.pos.get();
        if i < N::USIZE {
            let bufferp = self.buffer.get() as *mut T;
            unsafe { bufferp.add(i).write(value) }
            self.pos.set(i + 1);
            Ok(unsafe { &mut *bufferp.add(i) })
        } else {
            // OOM -- `Error` variant is not quite right but will make do
            Err(io::Error::NoMemory)
        }
    }
}
