//! Check that littlefs2 behaves sanely in an edge case
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example) and format the
//! partition (`emmc-fs-format` example) before running this; otherwise you'll run into a "corrupted
//! filesystem" error

#![no_main]
#![no_std]

use core::convert::TryInto;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{self, File, Fs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILENAME: &[u8] = b"baz.txt\0";
static TESTSTR: &[u8] = b"Hello File!";

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mbr = MbrDevice::open(emmc).unwrap();
    let part = mbr.into_partition(0).unwrap();

    let format = false;
    let f = Fs::mount(part, format).unwrap();
    memlog!("fs mounted");

    let filename = FILENAME.try_into().unwrap();
    let mut f1 = File::create(f, filename).unwrap();
    memlog!("file created");
    f1.write(TESTSTR).unwrap();
    memlog!("wrote data to file (but not yet committed it to disk)");

    fs::remove(f, filename).unwrap();
    memlog!("removed file from disk");

    if let Err(e) = File::open(f, filename) {
        if e == fs::Error::NoSuchEntry {
            memlog!("file doesn't exist (as expected)");
        } else {
            panic!("{:?}", e);
        }
    } else {
        panic!("file exists on disk");
    }

    memlog!("DONE");

    // then reset the board
    memlog_flush_and_reset!()
}
