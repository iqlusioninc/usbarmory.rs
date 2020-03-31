//! Partition table and block device access.

use core::{fmt, mem::size_of};

use memlog::memlog;
use zerocopy::{AsBytes, FromBytes, LayoutVerified};

/// Trait for block devices that can read, write, and erase 512-Byte blocks.
///
/// This is meant to be implemented for "managed" devices that have their own controller for
/// scheduling page erases and doing wear leveling, such as SD and MMC cards used by the Armory.
pub trait ManagedBlockDevice {
    /// The error type used by the block device implementation.
    type Error: fmt::Debug + fmt::Display;

    /// Returns the total number of 512-Byte blocks on the device.
    fn total_blocks(&self) -> u64;

    /// Reads a single block from the device.
    ///
    /// The `lba` parameter indicates the linera block address to write to. If it is outside of the
    /// valid range, an error must be returned.
    fn read(&self, block: &mut Block, lba: u64) -> Result<(), Self::Error>;

    /// Writes a single block to the device.
    ///
    /// The `lba` parameter indicates the linera block address to write to. If it is outside of the
    /// valid range, an error must be returned.
    ///
    /// This may write to a buffer and not to persistent storage. `flush` may be used to write all
    /// buffered data to persistent storage.
    fn write(&mut self, block: &Block, lba: u64) -> Result<(), Self::Error>;

    /// Flushes all buffered writes to persistent storage.
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<'a, D: ManagedBlockDevice> ManagedBlockDevice for &'a mut D {
    type Error = D::Error;

    fn total_blocks(&self) -> u64 {
        (**self).total_blocks()
    }

    fn read(&self, block: &mut Block, lba: u64) -> Result<(), Self::Error> {
        (**self).read(block, lba)
    }

    fn write(&mut self, block: &Block, lba: u64) -> Result<(), Self::Error> {
        (**self).write(block, lba)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        (**self).flush()
    }
}

/// Block size used by the storage subsystem.
///
/// SD and eMMC cards use 512 Byte blocks, which is convenient.
pub const BLOCK_SIZE: u16 = 512;

/// A copy of an eMMC/SD card block.
#[repr(align(4))]
#[derive(Clone)]
pub struct Block {
    /// The bytes contained in the memory block.
    pub bytes: [u8; BLOCK_SIZE as usize],
}

impl Block {
    /// Creates a `Block` buffer and initializes it to all zeros.
    pub fn zeroed() -> Self {
        Self {
            bytes: [0; BLOCK_SIZE as usize],
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::zeroed()
    }
}

/// Wraps an MBR-partitioned `ManagedBlockDevice` and provides access to the primary partitions.
pub struct MbrDevice<D: ManagedBlockDevice> {
    raw: D,
    part_table: [PartitionEntry; 4],
}

impl<D: ManagedBlockDevice> MbrDevice<D> {
    /// Creates a new MBR-partitioned block device by writing the given partition `table` into it
    pub fn create(mut raw: D, part_table: &PartitionTable) -> Result<Self, MbrError<D::Error>> {
        let total_blocks = raw.total_blocks();
        let end = part_table
            .as_slice()
            .last()
            .map(|entry| {
                let extent = entry.extent();
                extent.start + extent.sectors
            })
            .unwrap_or(0);

        if u64::from(end) > total_blocks {
            memlog!("PART end = {} > total_blocks = {}", end, total_blocks);
            return Err(MbrError::InvalidPartExtent);
        }

        let mbr = part_table.to_block();
        raw.write(&mbr, 0).map_err(MbrError::Device)?;
        Ok(MbrDevice { raw, part_table: part_table.entries })
    }

    /// Opens an MBR-partitioned block device `raw` and parses the partition table.
    pub fn open(raw: D) -> Result<Self, MbrError<D::Error>> {
        let mut mbr = Block::zeroed();
        raw.read(&mut mbr, 0).map_err(MbrError::Device)?;

        if mbr.bytes[usize::from(BLOCK_SIZE - 2)..] != [0x55, 0xAA] {
            return Err(MbrError::InvalidMagic);
        }

        // Copy the partition table to an aligned offset within the block (normally it's only at a
        // 2-aligned offset).
        mbr.bytes.copy_within(446..usize::from(BLOCK_SIZE - 2), 0);
        let parts: &[PartitionEntry] =
            LayoutVerified::new_slice(&mbr.bytes[..size_of::<PartitionEntry>() * 4])
                .unwrap()
                .into_slice();
        let mut part_table = [PartitionEntry::zeroed(); 4];
        part_table.copy_from_slice(parts);

        memlog!("MBR parttable: {:?}", parts);

        for part in parts {
            if part.part_type != 0x00 {
                // Entry allocated, extent must be valid.

                let start = part.extent().start;
                let end = u64::from(part.extent().start) + u64::from(part.extent().sectors);

                if start == 0 || u64::from(start) >= raw.total_blocks() {
                    memlog!("PART start = {}", start);
                    return Err(MbrError::InvalidPartExtent);
                }

                if end == 0 || end >= raw.total_blocks() {
                    memlog!("PART end = {}", end);
                    return Err(MbrError::InvalidPartExtent);
                }

                if end > u64::from(u32::max_value()) {
                    memlog!("PART end = {} (> u32 limit)", end);
                    return Err(MbrError::InvalidPartExtent);
                }
            }
        }

        Ok(Self { raw, part_table })
    }

    /// Obtains access to the partition at index `part` (0 ..= 3).
    ///
    /// Returns a `NoPartition` error if `part` does not refer to an allocated partition.
    pub fn partition(&mut self, part: u8) -> Result<MbrPartitionRef<'_, D>, MbrError<D::Error>> {
        let extent = self.part_extent(part)?;

        Ok(MbrPartitionRef {
            raw: &mut self.raw,
            extent,
        })
    }

    /// Returns a debug view into the partition table
    pub fn debug<'s>(&'s self) -> impl fmt::Debug + 's {
        &self.part_table
    }

    fn part_extent(&self, part: u8) -> Result<PartExtent, MbrError<D::Error>> {
        if part >= 4 {
            return Err(MbrError::NoPartition);
        }

        let entry = &self.part_table[usize::from(part)];
        if entry.part_type == 0x00 {
            // Entry unallocated.
            return Err(MbrError::NoPartition);
        }

        Ok(entry.extent())
    }
}

/// An entry in the MBR partition table
#[derive(AsBytes, FromBytes, Debug, Copy, Clone)]
#[repr(C)]
pub struct PartitionEntry {
    status: u8,
    start_chs: [u8; 3],
    part_type: u8,
    end_chs: [u8; 3],
    start_lba: u32,
    num_sectors: u32,
}

impl PartitionEntry {
    /// Creates a new partition entry that starts at `start_lba` and it's `num_sectors` big
    pub fn new(start_lba: u32, num_sectors: u32) -> Self {
        // this is what `fdisk` uses by default; it doesn't really matter in our case
        const PART_TYPE_LINUX: u8 = 0x83;

        Self {
            status: 0,
            // TODO we should enter something sensible here
            start_chs: [0; 3],
            // and here
            end_chs: [0; 3],
            part_type: PART_TYPE_LINUX,
            start_lba,
            num_sectors,
        }
    }

    /// Returns a zeroed/unallocated partition table entry.
    fn zeroed() -> Self {
        Self {
            status: 0,
            start_chs: [0; 3],
            part_type: 0,
            end_chs: [0; 3],
            start_lba: 0,
            num_sectors: 0,
        }
    }

    fn extent(&self) -> PartExtent {
        PartExtent {
            start: self.start_lba,
            sectors: self.num_sectors,
        }
    }
}

/// MBR partition table
pub struct PartitionTable {
    entries: [PartitionEntry; 4],
    index: usize,
}

impl PartitionTable {
    /// Creates an empty partition table
    pub fn new() -> Self {
        Self {
            entries: [PartitionEntry::zeroed(); 4],
            index: 0,
        }
    }

    fn as_slice(&self) -> &[PartitionEntry] {
        &self.entries[..self.index]
    }

    /// Adds a new partition to the table
    ///
    /// NOTE partitions must be added in order (increasing `start_lba`)
    pub fn add(&mut self, entry: PartitionEntry) -> Result<(), PartError> {
        let end = self
            .as_slice()
            .last()
            .map(|entry| entry.start_lba + entry.num_sectors)
            .unwrap_or(0);

        if entry.start_lba < end {
            Err(PartError::PartitionCollision)
        } else {
            if self.index < self.entries.len() {
                self.entries[self.index] = entry;
                Ok(())
            } else {
                Err(PartError::TooManyPartitions)
            }
        }
    }

    fn to_block(&self) -> Block {
        let mut block = Block::zeroed();
        let mut start = 446;
        for entry in &self.entries {
            let bytes = entry.as_bytes();
            let len = bytes.len();
            block.bytes[start..start + len].copy_from_slice(bytes);
            start += len;
        }
        // magic number
        block.bytes[510] = 0x55;
        block.bytes[511] = 0xAA;
        block
    }
}

/// Partition error
#[derive(Debug)]
pub enum PartError {
    /// New partition collides with an existing collision
    PartitionCollision,
    /// The partition table has already 4 primary partitions
    TooManyPartitions,
}

struct PartExtent {
    start: u32,
    sectors: u32,
}

/// Errors that can occur while opening or accessing an MBR-formatted block device.
#[derive(Debug)]
pub enum MbrError<D> {
    /// Error while accessing the underlying device.
    Device(D),

    /// The MBR had an invalid signature (did not end with `0x55 0xAA`).
    InvalidMagic,

    /// Encountered partition with invalid location/extent.
    InvalidPartExtent,

    /// Attempted to access partition that isn't allocated.
    NoPartition,

    /// Attempted to access block outside of device/partition.
    OutOfRangeAccess,
}

impl<D: fmt::Display> fmt::Display for MbrError<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MbrError::Device(err) => write!(f, "I/O error: {}", err),
            MbrError::InvalidMagic => f.write_str("MBR signature invalid"),
            MbrError::InvalidPartExtent => f.write_str("invalid partition entry (corrupted MBR?)"),
            MbrError::NoPartition => f.write_str("invalid partition index"),
            MbrError::OutOfRangeAccess => f.write_str("block access outside of valid range"),
        }
    }
}

/// Provides borrowed access to an MBR partition.
///
/// This implements `ManagedBlockDevice` and maps any access to the partition.
pub struct MbrPartitionRef<'a, D: ManagedBlockDevice> {
    raw: &'a mut D,
    extent: PartExtent,
}

impl<'a, D: ManagedBlockDevice> ManagedBlockDevice for MbrPartitionRef<'a, D> {
    type Error = MbrError<D::Error>;

    fn total_blocks(&self) -> u64 {
        self.extent.sectors.into()
    }

    fn read(&self, block: &mut Block, lba: u64) -> Result<(), Self::Error> {
        if lba >= u64::from(self.extent.sectors) {
            return Err(MbrError::OutOfRangeAccess);
        }

        self.raw
            .read(block, lba + u64::from(self.extent.start))
            .map_err(MbrError::Device)
    }

    fn write(&mut self, block: &Block, lba: u64) -> Result<(), Self::Error> {
        if lba >= u64::from(self.extent.sectors) {
            return Err(MbrError::OutOfRangeAccess);
        }

        self.raw
            .write(block, lba + u64::from(self.extent.start))
            .map_err(MbrError::Device)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.raw.flush().map_err(MbrError::Device)
    }
}
