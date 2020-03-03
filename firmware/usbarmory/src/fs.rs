//! File system access.

use crate::storage::{Block, ManagedBlockDevice};
use littlefs2::{
    consts,
    driver::Storage,
    fs::{Filesystem, FilesystemAllocation},
};
use memlog::memlog;

const BLOCK_COUNT: usize = 30_580_736;

/// Backing storage used by littlefs.
pub struct LittleFsAlloc<D: ManagedBlockDevice> {
    inner: FilesystemAllocation<LfsStorage<D>>,
}

impl<D: ManagedBlockDevice> LittleFsAlloc<D> {
    pub fn new() -> Self {
        Self {
            inner: Filesystem::allocate(),
        }
    }
}

/// A littlefs2 file system.
pub struct LittleFs<'a, D: ManagedBlockDevice> {
    storage: LfsStorage<D>,
    fs: Filesystem<'a, LfsStorage<D>>,
}

impl<'a, D: ManagedBlockDevice> LittleFs<'a, D> {
    /// Mounts a littlefs2 file system.
    pub fn mount(alloc: &'a mut LittleFsAlloc<D>, blockdev: D) -> littlefs2::io::Result<Self> {
        if blockdev.total_blocks() != BLOCK_COUNT as u64 {
            memlog!(
                "expected {} blocks, got {}",
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
    pub fn with<R>(
        blockdev: D,
        f: impl FnOnce(&mut LittleFs<'_, D>) -> R,
    ) -> littlefs2::io::Result<R> {
        let mut alloc = LittleFsAlloc::new();
        let mut fs = LittleFs::mount(&mut alloc, blockdev)?;

        Ok(f(&mut fs))
    }

    /// Formats `blockdev`, creating a fresh littlefs file system (this erases all data!).
    pub fn format(blockdev: D) -> littlefs2::io::Result<()> {
        Filesystem::format(&mut LfsStorage { inner: blockdev })
    }

    /// Returns the available space in Bytes (approximated).
    pub fn available_space(&mut self) -> littlefs2::io::Result<u64> {
        self.fs.available_space(&mut self.storage).map(|space| space as u64)
    }
}

pub struct File {}

impl File {
    pub fn with<R, D: ManagedBlockDevice>(
        fs: &mut LittleFs<'_, D>,
        path: impl AsRef<[u8]>,
        f: impl FnOnce(&mut File) -> R,
    ) -> littlefs2::io::Result<R> {
        todo!()
    }
}

struct LfsStorage<D: ManagedBlockDevice> {
    inner: D,
}

impl<D: ManagedBlockDevice> Storage for LfsStorage<D> {
    type CACHE_SIZE = consts::U512;
    type LOOKAHEADWORDS_SIZE = consts::U16;
    type FILENAME_MAX_PLUS_ONE = consts::U256;
    type PATH_MAX_PLUS_ONE = consts::U256;
    type ATTRBYTES_MAX = consts::U1022;

    const READ_SIZE: usize = 512;
    const WRITE_SIZE: usize = 512;
    const BLOCK_SIZE: usize = 512;

    // FIXME: This really shouldn't be a constant. This value is only correct for the second
    // partition on the eMMC, and only when following the instructions *exactly*.
    const BLOCK_COUNT: usize = BLOCK_COUNT;

    // Disable wear leveling since the `ManagedBlockDevice` is assumed to already implement that.
    const BLOCK_CYCLES: isize = -1;
    const FILEBYTES_MAX: usize = 2_147_483_647;

    fn read(&self, off: usize, buf: &mut [u8]) -> littlefs2::io::Result<usize> {
        memlog!("read {} @ {:x}", buf.len(), off);

        let mut lba = off / Self::BLOCK_SIZE;

        let mut block = Block::zeroed();
        for buf_block in buf.chunks_mut(Self::BLOCK_SIZE) {
            self.inner
                .read(&mut block, lba as u64)
                .map_err(|_| littlefs2::io::Error::Io)?;
            buf_block.copy_from_slice(&block.bytes);
            lba += 1;
        }

        // XXX this is ignored
        Ok(buf.len())
    }

    fn write(&mut self, off: usize, data: &[u8]) -> littlefs2::io::Result<usize> {
        memlog!("write {} @ {:x}", data.len(), off);

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

        // XXX this is ignored
        Ok(data.len())
    }

    fn erase(&mut self, off: usize, len: usize) -> littlefs2::io::Result<usize> {
        // A `ManagedBlockDevice` can just overwrite individual blocks, no need to erase any.
        memlog!("erase {} @ {:x}", len, off);

        // XXX this is ignored
        Ok(len)
    }
}

/// Dummy storage used to avoid unnecessary type parameters.
struct LfsDummyStorage {}

impl Storage for LfsDummyStorage {
    type CACHE_SIZE = consts::U512;
    type LOOKAHEADWORDS_SIZE = consts::U16;
    type FILENAME_MAX_PLUS_ONE = consts::U256;
    type PATH_MAX_PLUS_ONE = consts::U256;
    type ATTRBYTES_MAX = consts::U1022;

    const READ_SIZE: usize = 512;
    const WRITE_SIZE: usize = 512;
    const BLOCK_SIZE: usize = 512;

    // FIXME: This really shouldn't be a constant. This value is only correct for the second
    // partition on the eMMC, and only when following the instructions *exactly*.
    const BLOCK_COUNT: usize = BLOCK_COUNT;

    // Disable wear leveling since the `ManagedBlockDevice` is assumed to already implement that.
    const BLOCK_CYCLES: isize = -1;
    const FILEBYTES_MAX: usize = 2_147_483_647;

    fn read(&self, _off: usize, _buf: &mut [u8]) -> littlefs2::io::Result<usize> {
        unimplemented!();
    }

    fn write(&mut self, _off: usize, _data: &[u8]) -> littlefs2::io::Result<usize> {
        unimplemented!();
    }

    fn erase(&mut self, _off: usize, _len: usize) -> littlefs2::io::Result<usize> {
        unimplemented!();
    }
}
