//! File system access.

use crate::storage::{Block, ManagedBlockDevice, BLOCK_SIZE};
use littlefs2::{
    consts,
    driver::Storage,
    fs::{self, FileAllocation, FileType, Filesystem, FilesystemAllocation, Metadata, SeekFrom},
    io::{self, Read, Seek, Write},
    path::Filename,
};
use memlog::memlog;

/// Hardcoded filesystem block count.
///
/// This should be removed and calculated dynamically based on the partition size.
///
/// littlefs2 has a hard 2^32 Byte limit.
const BLOCK_COUNT: usize = 131_072; // 64 MiB

/// Backing storage used by littlefs.
pub struct LittleFsAlloc<D: ManagedBlockDevice> {
    inner: FilesystemAllocation<LfsStorage<D>>,
}

impl<D: ManagedBlockDevice> LittleFsAlloc<D> {
    /// Creates a new filesystem allocation.
    pub fn new() -> Self {
        Self {
            inner: Filesystem::allocate(),
        }
    }
}

impl<D: ManagedBlockDevice> Default for LittleFsAlloc<D> {
    fn default() -> Self {
        Self::new()
    }
}

/// A littlefs2 file system.
pub struct LittleFs<'a, D: ManagedBlockDevice> {
    storage: LfsStorage<D>,
    fs: Filesystem<'a, LfsStorage<D>>,
}

impl<'a, D: ManagedBlockDevice> LittleFs<'a, D> {
    /// Mounts a littlefs2 file system.
    pub fn mount(alloc: &'a mut LittleFsAlloc<D>, blockdev: D) -> io::Result<Self> {
        if blockdev.total_blocks() < BLOCK_COUNT as u64 {
            memlog!(
                "expected at least {} blocks, got {}",
                BLOCK_COUNT,
                blockdev.total_blocks()
            );

            return Err(littlefs2::io::Error::NoSpace); // close enough?
        }

        let mut storage = LfsStorage { inner: blockdev };
        let fs = Filesystem::mount(&mut alloc.inner, &mut storage)?;

        Ok(Self { storage, fs })
    }

    /// Mounts a littlefs2 file system for the duration of a closure `f`.
    ///
    /// This API avoids the need for using `LittleFsAlloc`.
    pub fn mount_and_then<R>(
        blockdev: D,
        f: impl FnOnce(&mut LittleFs<'_, D>) -> io::Result<R>,
    ) -> io::Result<R> {
        let mut alloc = LittleFsAlloc::new();
        let mut fs = LittleFs::mount(&mut alloc, blockdev)?;

        f(&mut fs)
    }

    /// Formats `blockdev`, creating a fresh littlefs file system (this erases all data!).
    pub fn format(blockdev: D) -> io::Result<()> {
        Filesystem::format(&mut LfsStorage { inner: blockdev })
    }

    /// Returns the available space in Bytes (approximated).
    pub fn available_space(&mut self) -> io::Result<u64> {
        self.fs
            .available_space(&mut self.storage)
            .map(|space| space as u64)
    }

    /// Creates a new directory at `path`.
    pub fn create_dir(&mut self, path: impl AsRef<[u8]>) -> io::Result<()> {
        self.fs.create_dir(path.as_ref(), &mut self.storage)
    }

    /// Removes the file or directory at `path`.
    pub fn remove(&mut self, path: impl AsRef<[u8]>) -> io::Result<()> {
        self.fs.remove(path.as_ref(), &mut self.storage)
    }

    /// Returns an iterator over the contents of the directory at `path`.
    pub fn read_dir<'r>(&'r mut self, path: impl AsRef<[u8]>) -> io::Result<ReadDir<'r, 'a, D>> {
        self.fs
            .read_dir(path.as_ref(), &mut self.storage)
            .map(move |inner| ReadDir { fs: self, inner })
    }
}

/// Allocation backing a `File` instance.
pub struct FileAlloc<D: ManagedBlockDevice> {
    inner: FileAllocation<LfsStorage<D>>,
}

impl<D: ManagedBlockDevice> FileAlloc<D> {
    /// Creates a new file allocation.
    pub fn new() -> Self {
        Self {
            inner: fs::File::allocate(),
        }
    }
}

impl<D: ManagedBlockDevice> Default for FileAlloc<D> {
    fn default() -> Self {
        Self::new()
    }
}

/// An open file.
///
/// NOTE unlike `littlefs2::File`, this newtype has close on drop semantics. Any error that arises
/// while closing the file will result in a panic. Use the `close` method to handle IO errors
/// instead of potentially panicking.
pub struct File<'a, 'fs, D: ManagedBlockDevice> {
    inner: Option<fs::File<'a, LfsStorage<D>>>,
    fs: &'a mut LittleFs<'fs, D>,
}

#[allow(clippy::len_without_is_empty)]
impl<'a, 'fs, D: ManagedBlockDevice> File<'a, 'fs, D> {
    /// Opens the file at `path`.
    pub fn open(
        fs: &'a mut LittleFs<'fs, D>,
        alloc: &'a mut FileAlloc<D>,
        path: impl AsRef<[u8]>,
    ) -> io::Result<Self> {
        let mut inner =
            fs::File::open(path.as_ref(), &mut alloc.inner, &mut fs.fs, &mut fs.storage)?;
        inner.seek(&mut fs.fs, &mut fs.storage, SeekFrom::Start(0))?;
        Ok(Self {
            inner: Some(inner),
            fs,
        })
    }

    /// Creates or overwrites a file at `path`.
    pub fn create(
        fs: &'a mut LittleFs<'fs, D>,
        alloc: &'a mut FileAlloc<D>,
        path: impl AsRef<[u8]>,
    ) -> io::Result<Self> {
        Ok(Self {
            inner: Some(fs::File::create(
                path.as_ref(),
                &mut alloc.inner,
                &mut fs.fs,
                &mut fs.storage,
            )?),
            fs,
        })
    }

    /// Calls a closure with the file at `path`.
    ///
    /// This avoids having to use `FileAlloc`.
    ///
    /// NOTE the file will be `sync`-ed and `close`-d after `f` is executed
    pub fn open_and_then<R>(
        fs: &mut LittleFs<'a, D>,
        path: impl AsRef<[u8]>,
        f: impl FnOnce(&mut File<'_, '_, D>) -> io::Result<R>,
    ) -> io::Result<R> {
        let mut alloc = FileAlloc::new();
        let mut file = File::open(fs, &mut alloc, path)?;

        let res = f(&mut file);
        file.close()?;
        res
    }

    /// Calls a closure with a file created at `path`.
    ///
    /// This avoids having to use `FileAlloc`.
    ///
    /// NOTE the file will be `sync`-ed and `close`-d after `f` is executed
    pub fn create_and_then<R>(
        fs: &mut LittleFs<'a, D>,
        path: impl AsRef<[u8]>,
        f: impl FnOnce(&mut File<'_, '_, D>) -> io::Result<R>,
    ) -> io::Result<R> {
        let mut alloc = FileAlloc::new();
        let mut file = File::create(fs, &mut alloc, path)?;

        let r = f(&mut file)?;
        file.close()?;
        Ok(r)
    }

    /// Consumes and closes the file.
    ///
    /// This will also synchronize the contents of the file to disk (i.e. flush the file write
    /// cache)
    ///
    /// NOTE the file will also be closed when dropped; but you can use this method to handle IO
    /// errors that may occur while closing the file
    pub fn close(mut self) -> io::Result<()> {
        self.inner
            .take()
            .unwrap_or_else(|| unsafe { assume_unreachable!() })
            .close(&mut self.fs.fs, &mut self.fs.storage)
    }

    /// Returns the length of this file in Bytes.
    pub fn len(&mut self) -> io::Result<usize> {
        self.inner
            .as_mut()
            .unwrap_or_else(|| unsafe { assume_unreachable!() })
            .len(&mut self.fs.fs, &mut self.fs.storage)
    }

    /// Reads bytes from this file into `buf`.
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner
            .as_mut()
            .unwrap_or_else(|| unsafe { assume_unreachable!() })
            .read(&mut self.fs.fs, &mut self.fs.storage, buf)
    }

    /// Writes byte from `buf` into this file.
    ///
    /// NOTE writes are cached in memory; use `sync` to flush the cache to disk
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner
            .as_mut()
            .unwrap_or_else(|| unsafe { assume_unreachable!() })
            .write(&mut self.fs.fs, &mut self.fs.storage, buf)
    }

    /// Synchronize file contents to storage
    pub fn sync(&mut self) -> io::Result<()> {
        self.inner
            .as_mut()
            .unwrap_or_else(|| unsafe { assume_unreachable!() })
            .sync(&mut self.fs.fs, &mut self.fs.storage)
    }
}

impl<D> Drop for File<'_, '_, D>
where
    D: ManagedBlockDevice,
{
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            inner.close(&mut self.fs.fs, &mut self.fs.storage).unwrap()
        }
    }
}

/// An iterator over entries in a directory.
pub struct ReadDir<'a, 'fs, D: ManagedBlockDevice> {
    inner: fs::ReadDir<LfsStorage<D>>,
    fs: &'a mut LittleFs<'fs, D>,
}

impl<'a, 'fs, D: ManagedBlockDevice> Iterator for ReadDir<'a, 'fs, D> {
    type Item = littlefs2::io::Result<DirEntry<D>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next(&mut self.fs.fs, &mut self.fs.storage) {
            Some(res) => Some(res.map(|inner| DirEntry { inner })),
            None => None,
        }
    }
}

/// A directory entry returned by `ReadDir`.
pub struct DirEntry<D: ManagedBlockDevice> {
    inner: fs::DirEntry<LfsStorage<D>>,
}

impl<D: ManagedBlockDevice> DirEntry<D> {
    /// Returns the type of this entry.
    pub fn file_type(&self) -> FileType {
        self.inner.file_type()
    }

    /// Returns the name of this entry
    pub fn file_name(&self) -> Filename<LfsStorage<D>> {
        self.inner.file_name()
    }

    /// Returns the metadata of this entry
    pub fn metadata(&self) -> Metadata {
        self.inner.metadata()
    }
}

#[doc(hidden)]
pub struct LfsStorage<D: ManagedBlockDevice> {
    inner: D,
}

impl<D: ManagedBlockDevice> Storage for LfsStorage<D> {
    type CACHE_SIZE = consts::U512;
    type LOOKAHEADWORDS_SIZE = consts::U16;
    type FILENAME_MAX_PLUS_ONE = consts::U256;
    type PATH_MAX_PLUS_ONE = consts::U256;
    type ATTRBYTES_MAX = consts::U1022;

    const READ_SIZE: usize = BLOCK_SIZE as usize;
    const WRITE_SIZE: usize = BLOCK_SIZE as usize;
    const BLOCK_SIZE: usize = BLOCK_SIZE as usize;

    // FIXME: This really shouldn't be a constant.
    const BLOCK_COUNT: usize = BLOCK_COUNT;

    // Disable wear leveling since the `ManagedBlockDevice` is assumed to already implement that.
    const BLOCK_CYCLES: isize = -1;
    const FILEBYTES_MAX: usize = 2_147_483_647;

    fn read(&self, off: usize, buf: &mut [u8]) -> littlefs2::io::Result<usize> {
        let mut lba = off / Self::BLOCK_SIZE;

        let mut block = Block::zeroed();
        for buf_block in buf.chunks_mut(Self::BLOCK_SIZE) {
            self.inner
                .read(&mut block, lba as u64)
                .map_err(|_| littlefs2::io::Error::Io)?;
            buf_block.copy_from_slice(&block.bytes);
            lba += 1;
        }

        Ok(buf.len())
    }

    fn write(&mut self, off: usize, data: &[u8]) -> littlefs2::io::Result<usize> {
        let mut lba = off / Self::BLOCK_SIZE;

        let mut block = Block::zeroed();
        for buf_block in data.chunks(Self::BLOCK_SIZE) {
            block.bytes.copy_from_slice(buf_block);
            self.inner
                .write(&block, lba as u64)
                .map_err(|_| littlefs2::io::Error::Io)?;
            lba += 1;
        }

        self.inner.flush().map_err(|_| littlefs2::io::Error::Io)?;

        Ok(data.len())
    }

    fn erase(&mut self, _off: usize, len: usize) -> littlefs2::io::Result<usize> {
        // A `ManagedBlockDevice` can just overwrite individual blocks, no need to erase any.
        Ok(len)
    }
}
