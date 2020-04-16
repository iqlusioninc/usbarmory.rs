//! Prints partition sizes of the MBR-formatted eMMC.

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    memlog, memlog_flush_and_reset,
    storage::{ManagedBlockDevice, MbrDevice, BLOCK_SIZE},
};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();
    let mbr = MbrDevice::open(emmc).unwrap();

    memlog!("{:#?}", mbr.debug());

    if let Ok(part) = mbr.into_partition(0) {
        memlog!(
            "first partition is {} MiB",
            (part.total_blocks() * u64::from(BLOCK_SIZE)) / 1024 / 1024
        );
    }

    // then reset the board
    memlog_flush_and_reset!();
}
