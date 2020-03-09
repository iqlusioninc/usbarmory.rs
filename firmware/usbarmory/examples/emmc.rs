//! Basic persistent storage using the eMMC
//!
//! Reads the first (512B) memory block on the eMMC, increases the value of the
//! first byte of the sector and then writes the updated sector back into the
//! card. The byte stored in non-volatile memory keeps a count of how many times
//! this program has run.

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{emmc::eMMC, memlog, memlog_flush_and_reset, storage::Block};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC");

    let mut block = Block::zeroed();
    emmc.read(0, &mut block);

    memlog!("first byte of the first block: {}", block.bytes[0]);
    block.bytes[0] += 1;

    emmc.write(0, &block);
    emmc.flush();

    // then reset the board to return to the u-boot console
    memlog_flush_and_reset!();
}
