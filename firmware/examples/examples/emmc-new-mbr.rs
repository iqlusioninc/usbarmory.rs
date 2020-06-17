//! Creates a new MBR partition table on the eMMC

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    memlog, memlog_flush_and_reset,
    storage::{MbrDevice, PartitionEntry, PartitionTable, BLOCK_SIZE},
};

#[allow(non_upper_case_globals)]
const MiB: u32 = 1024 * 1024;
/// Start sector = 32 MiB; those first 32 MiB are reserved for the boot image
const START: u32 = 32 * MiB / BLOCK_SIZE as u32;
/// Size of the partition in sectors
const SIZE: u32 = 100 * MiB / BLOCK_SIZE as u32;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let mut table = PartitionTable::new();
    table.add(PartitionEntry::new(START, SIZE)).unwrap();
    let emmc = eMMC::take().expect("eMMC").unwrap();
    let mbr = MbrDevice::create(emmc, &table).unwrap();

    memlog!("{:#?}", mbr.debug());

    // then reset the board
    memlog_flush_and_reset!();
}
