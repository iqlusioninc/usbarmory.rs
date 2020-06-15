//! Sanity check that the `fs` API works as expected
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
    fs::{self, Fs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

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

    let foo = b"foo\0".try_into().unwrap();
    let res = fs::create_dir(f, foo);

    if res != Err(fs::Error::EntryAlreadyExisted) {
        res.unwrap();
        memlog!("created directory `foo`");
    } else {
        memlog!("directory `foo` already exists");
    }

    let res = fs::create_dir(f, b"foo/bar\0".try_into().unwrap());
    if res != Err(fs::Error::EntryAlreadyExisted) {
        res.unwrap();
        memlog!("created directory `foo/bar`");
    } else {
        memlog!("directory `foo/bar` already exists");
    }

    memlog!("iterating over the contents of directory `foo`");

    for (i, entry) in fs::read_dir(f, foo).unwrap().enumerate() {
        let entry = entry.unwrap();
        // NOTE omitted the name because it gets `Debug` printed as an array
        memlog!("{}: {:?}", i, entry.metadata());
    }

    // then reset the board
    memlog_flush_and_reset!();
}
