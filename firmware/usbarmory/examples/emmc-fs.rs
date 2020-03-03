#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
    fs::LittleFs,
};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC");
    for _ in 0..4 { usbarmory::memlog_try_flush(); }

    let mut mbr = MbrDevice::open(emmc).unwrap();
    let mut main_part = mbr.partition(1).unwrap();
    for _ in 0..4 { usbarmory::memlog_try_flush(); }

    if let Err(littlefs2::io::Error::Corruption) = LittleFs::with(&mut main_part, |_| {}) {
        memlog!("Formatting disk");
        LittleFs::format(&mut main_part).unwrap();
    }

    for _ in 0..4 { usbarmory::memlog_try_flush(); }
    LittleFs::with(main_part, |fs| -> littlefs2::io::Result<()> {
        memlog!("{} bytes free", fs.available_space()?);
        Ok(())
    }).unwrap().unwrap();

    // then reset the board to return to the u-boot console
    memlog_flush_and_reset!();
}
