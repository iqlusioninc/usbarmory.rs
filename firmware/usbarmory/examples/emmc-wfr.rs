//! eMMC: check that write, flush, read works as expected

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

    let mut orig = Block::zeroed();
    emmc.read(BLOCK_NR, &mut orig).unwrap();

    let mut updated = orig.clone();

    updated
        .bytes
        .iter_mut()
        .for_each(|byte| *byte = byte.wrapping_add(1));

    emmc.write(BLOCK_NR, &updated).unwrap();
    emmc.flush().unwrap();

    let mut fresh = Block::zeroed();
    emmc.read(BLOCK_NR, &mut fresh).unwrap();

    if updated.bytes[..] == fresh.bytes[..] {
        memlog!("OK @ {:#x}", BLOCK_NR);
    } else {
        memlog!("error @ {:#x}: blocks don't match", BLOCK_NR);

        for i in 0..fresh.bytes.len() {
            if updated.bytes[i] != fresh.bytes[i] {
                memlog!(
                    "@ {:#04x}: initial: {:#04x}, got: {:#04x}, expected: {:#04x}",
                    i,
                    orig.bytes[i],
                    fresh.bytes[i],
                    updated.bytes[i],
                );
            }
        }
    }

    // then reset the board to return to the u-boot console
    memlog_flush_and_reset!();
}
