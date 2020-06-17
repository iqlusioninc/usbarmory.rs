//! eMMC: check that write, flush, read works as expected
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
// like `#[rtic::app]`
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

    // NOTE all writes are immediately flushed to the eMMC

    let mut fresh = Block::zeroed();
    emmc.read(BLOCK_NR, &mut fresh).unwrap();

    if updated.bytes[..] == fresh.bytes[..] {
        memlog!("OK @ {:#x}", BLOCK_NR);
    } else {
        let mut total = 0;
        let mut first = None;
        let mut last = 0;
        for i in 0..fresh.bytes.len() {
            if updated.bytes[i] != fresh.bytes[i] {
                if first.is_none() {
                    first = Some(i);
                }
                total += 1;
                last = i;
            }
        }

        memlog!(
            "error @ {:#x}: blocks don't match ({}B differ, first={}, last={})",
            BLOCK_NR,
            total,
            first.unwrap(),
            last
        );
    }

    // then reset the board
    memlog_flush_and_reset!();
}
