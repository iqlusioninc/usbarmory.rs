//! Check that littlefs2 behaves sanely in an edge case
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example) and format the
//! partition (`emmc-fs-format` example) before running this; otherwise you'll run into a "corrupted
//! filesystem" error

#![no_main]
#![no_std]

use core::str;

use exception_reset as _; // default exception handler
use littlefs2::io::{self, Error};
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{File, FileAlloc, LittleFs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILENAME: &str = "baz.txt";
static TESTSTR: &[u8] = b"Hello File!";

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

        let mut f = FileAlloc::new();
        let f = File::create(fs, &mut f, FILENAME)?;
        memlog!("file created");
        f.write(TESTSTR)?;
        memlog!("wrote data to file (but not yet committed it to disk)");

        fs.remove(FILENAME)?;
        memlog!("removed file from disk");

        f.close()?;
        memlog!("closed file handle");

        let mut f = FileAlloc::new();
        if let Err(e) = File::open(fs, &mut f, FILENAME) {
            if e == Error::NoSuchEntry {
                memlog!("file doesn't exist (as expected)");
            } else {
                return Err(e);
            }
        } else {
            panic!("file exists on disk");
        }

        Ok(())
    })
    .unwrap();

    memlog!("DONE");

    // then reset the board
    memlog_flush_and_reset!()
}
