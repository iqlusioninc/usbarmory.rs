//! Sanity check that the `File` API works as expected
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example) and format the
//! partition (`emmc-fs-format` example) before running this; otherwise you'll run into a "corrupted
//! filesystem" error

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use littlefs2::io;
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{File, LittleFs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILENAME: &str = "hello.txt";
static TESTSTR: &[u8] = b"Hello File!";

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mut mbr = MbrDevice::open(emmc).unwrap();
    let mut main_part = mbr.partition(0).unwrap();

    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        memlog!("fs mounted");

        let mut success = false;
        File::create_and_then(fs, FILENAME, |file| -> io::Result<()> {
            success = true;
            memlog!("file created");
            file.write(TESTSTR)?;
            memlog!("wrote data to file (but have not yet committed it to disk)");
            Ok(())
        })?;

        if success {
            memlog!("file committed to disk");
        }

        Ok(())
    })
    .unwrap();

    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        memlog!("fs mounted");

        File::open_and_then(fs, FILENAME, |file| -> io::Result<()> {
            memlog!("file opened");

            assert_eq!(
                file.len()?,
                TESTSTR.len(),
                "file length doesn't match our expectations"
            );

            let mut buf = [0; 32];
            let n = file.read(&mut buf)?;
            assert_eq!(
                &buf[..n],
                TESTSTR,
                "file contents don't match our expectations"
            );
            Ok(())
        })?;

        memlog!("all OK");

        Ok(())
    })
    .unwrap();

    // then reset the board
    memlog_flush_and_reset!();
}
