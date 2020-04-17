//! File system access.
//!
//! The filesystem can be configured, via `const`s, in the following files:
//!
//! - `firmware/usbarmory/src/fs.rs`
//! - `common/littlefs/src/consts.rs`

pub use littlefs::{fs::*, io::Error};
use littlefs::{filesystem, io, storage::Storage};

use crate::{
    emmc::eMMC,
    storage::{Block, ManagedBlockDevice, MbrPartition, BLOCK_SIZE},
};

// NOTE end-users should only modify these constants
const READ_DIR_DEPTH: usize = 2;
const MAX_OPEN_FILES: usize = 4;

/// Hardcoded filesystem block count.
///
/// This should be removed and calculated dynamically based on the partition size.
///
/// littlefs2 has a hard 2^32 Byte limit.
// NOTE if you modify this you may need to modify the size of the MBR partition; the MBR partition
// must be bigger than this number
const BLOCK_COUNT: u32 = 131_072; // 64 MiB / 512 (=block_size)

filesystem!(
    /// Filesystem backed by an eMMC
    Fs,
    Storage = MbrPartition<eMMC>,
    max_open_files = MAX_OPEN_FILES,
    read_dir_depth = READ_DIR_DEPTH
);

unsafe impl<D> Storage for MbrPartition<D>
where
    D: ManagedBlockDevice,
{
    // FIXME: This really shouldn't be a constant.
    const BLOCK_COUNT: u32 = BLOCK_COUNT;

    fn read(&self, off: usize, buf: &mut [u8]) -> io::Result<()> {
        let mut lba = off / usize::from(BLOCK_SIZE);

        let mut block = Block::zeroed();
        for buf_block in buf.chunks_mut(BLOCK_SIZE.into()) {
            ManagedBlockDevice::read(self, &mut block, lba as u64).map_err(|_| io::Error::Io)?;
            buf_block.copy_from_slice(&block.bytes);
            lba += 1;
        }

        Ok(())
    }

    fn write(&self, off: usize, data: &[u8]) -> io::Result<()> {
        if self.lock.get() {
            return Err(io::Error::WriteWhileLocked);
        }

        let mut lba = off / usize::from(BLOCK_SIZE);

        let mut block = Block::zeroed();
        for buf_block in data.chunks(BLOCK_SIZE.into()) {
            block.bytes.copy_from_slice(buf_block);
            ManagedBlockDevice::write(self, &block, lba as u64).map_err(|_| io::Error::Io)?;
            lba += 1;
        }

        Ok(())
    }

    fn erase(&self, _off: usize, _len: usize) -> io::Result<()> {
        // A `ManagedBlockDevice` can just overwrite individual blocks, no need to erase any.
        Ok(())
    }

    fn lock(&self) {
        self.lock.set(true)
    }

    fn unlock(&self) {
        self.lock.set(false)
    }
}
