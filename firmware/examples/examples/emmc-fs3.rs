//! Check that we can work with multiple opened files
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
    fs::{File, FileAlloc, LittleFs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILE1: &str = "foo.txt";
static FILE2: &str = "bar.txt";
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

        let mut f1 = FileAlloc::new();
        let f1 = File::create(fs, &mut f1, FILE1)?;
        memlog!("file1 created");
        f1.write(TESTSTR)?;
        memlog!("wrote data to file1");

        let mut f2 = FileAlloc::new();
        let f2 = File::create(fs, &mut f2, FILE2)?;
        memlog!("file2 created");

        f2.write(TESTSTR)?;
        memlog!("wrote data to file2");

        Ok(())
    })
    .unwrap();

    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        memlog!("fs mounted");

        let mut f1 = FileAlloc::new();
        let mut f2 = FileAlloc::new();

        let f1 = File::open(fs, &mut f1, FILE1)?;
        memlog!("file1 opened");

        let f2 = File::open(fs, &mut f2, FILE2)?;
        memlog!("file2 opened");

        for f in [f1, f2].iter() {
            assert_eq!(
                f.len()?,
                TESTSTR.len(),
                "file length doesn't match our expectations"
            );

            let mut buf = [0; 32];
            let n = f.read(&mut buf)?;
            assert_eq!(
                &buf[..n],
                TESTSTR,
                "file contents don't match our expectations"
            );
        }

        memlog!("all OK");

        Ok(())
    })
    .unwrap();

    // then reset the board
    memlog_flush_and_reset!();
}
