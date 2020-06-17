//! Sanity check that the `fs` API works as expected
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example) and format the
//! partition (`emmc-fs-format` example) before running this; otherwise you'll run into a "corrupted
//! filesystem" error

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use littlefs2::io;
use littlefs2::io::Error;
use panic_serial as _; // panic handler
use usbarmory::{emmc::eMMC, fs::LittleFs, memlog, memlog_flush_and_reset, storage::MbrDevice};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mut mbr = MbrDevice::open(emmc).unwrap();
    let mut main_part = mbr.partition(0).unwrap();

    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        memlog!("fs mounted");

        let res = fs.create_dir("foo");
        if res != Err(Error::EntryAlreadyExisted) {
            memlog!("created directory `foo`");
            res?;
        } else {
            memlog!("directory `foo` already exists");
        }

        let res = fs.create_dir("foo/bar");
        if res != Err(Error::EntryAlreadyExisted) {
            memlog!("created directory `foo/bar`");
            res?;
        } else {
            memlog!("directory `foo/bar` already exists");
        }

        Ok(())
    })
    .unwrap();

    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        memlog!("fs mounted");

        memlog!("iterating over the contents of directory `foo`");
        for (i, entry) in fs.read_dir("foo")?.enumerate() {
            let entry = entry?;
            // NOTE omitted the name because it gets `Debug` printed as an array
            memlog!("{}: {:?}", i, entry.metadata());
        }

        Ok(())
    })
    .unwrap();

    // then reset the board
    memlog_flush_and_reset!();
}
