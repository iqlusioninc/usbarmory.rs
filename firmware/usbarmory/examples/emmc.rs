//! Basic persistent storage using the eMMC
//!
//! Reads one (512B) memory block, increases the value of the first byte of the sector and then
//! writes the updated sector back into the card. The byte stored in non-volatile memory keeps a
//! count of how many times this program has run.
//!
//! **WARNING** this may corrupt data on the eMMC; make a back up or double check that this won't
//! overwrite existing data

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{emmc::eMMC, memlog, memlog_flush_and_reset, storage::Block};

const BLOCK_NR: u32 = 204800; // an offset of 100MB

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mut block = Block::zeroed();
    emmc.read(BLOCK_NR, &mut block).unwrap();

    memlog!("first byte of block {:x}: {}", BLOCK_NR, block.bytes[0]);
    block.bytes[0] = block.bytes[0].wrapping_add(1);

    emmc.write(0, &block).unwrap();
    emmc.flush().unwrap();

    // then reset the board to return to the u-boot console
    memlog_flush_and_reset!();
}
