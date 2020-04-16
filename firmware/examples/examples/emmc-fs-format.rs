//! `littlefs2` formats the first MBR partition

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{emmc::eMMC, fs::Fs, memlog, memlog_flush_and_reset, storage::MbrDevice};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mbr = MbrDevice::open(emmc).unwrap();
    let part = mbr.into_partition(0).unwrap();

    let format = true;
    Fs::mount(part, format).unwrap();

    memlog!("formatting DONE");

    // then reset the board
    memlog_flush_and_reset!();
}
