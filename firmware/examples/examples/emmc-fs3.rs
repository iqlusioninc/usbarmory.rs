//! Check that we can work with multiple opened files
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

static FILE1: &[u8] = b"foo.txt\0";
static FILE2: &[u8] = b"bar.txt\0";
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

    let file1 = FILE1.try_into().unwrap();
    let file2 = FILE2.try_into().unwrap();

    let mut f1 = File::create(f, file1).unwrap();
    memlog!("file1 created");
    f1.write(TESTSTR).unwrap();
    memlog!("wrote data to file1");

    let mut f2 = File::create(f, file2).unwrap();
    memlog!("file2 created");

    f2.write(TESTSTR).unwrap();
    memlog!("wrote data to file2");

    // close files
    drop(f1);
    drop(f2);
    memlog!("closed files");

    let f1 = File::open(f, file1).unwrap();
    memlog!("file1 opened");

    let f2 = File::open(f, file2).unwrap();
    memlog!("file2 opened");

    let mut buf = [0; 32];
    for f in [f1, f2].iter_mut() {
        assert_eq!(
            f.len().unwrap(),
            TESTSTR.len(),
            "file length doesn't match our expectations"
        );

        let n = f.read(&mut buf).unwrap();
        assert_eq!(
            &buf[..n],
            TESTSTR,
            "file contents don't match our expectations"
        );
    }

    memlog!("all OK");

    // then reset the board
    memlog_flush_and_reset!();
}
