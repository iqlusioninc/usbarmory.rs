//! Sanity check that the `File` API works as expected
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
    fs::{File, Fs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILENAME: &[u8] = b"hello.txt\0";
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
    let mut file = File::create(f, filename).unwrap();
    memlog!("file created");

    file.write(TESTSTR).unwrap();
    memlog!("wrote data to file (but have not yet committed it to disk)");

    file.close().unwrap();
    memlog!("committed data to disk");

    let mut file = File::open(f, filename).unwrap();

    assert_eq!(
        file.len().unwrap(),
        TESTSTR.len(),
        "file length doesn't match our expectations"
    );

    let mut buf = [0; 32];
    let n = file.read(&mut buf).unwrap();
    assert_eq!(
        &buf[..n],
        TESTSTR,
        "file contents don't match our expectations"
    );

    memlog!("all OK");

    // then reset the board
    memlog_flush_and_reset!();
}
