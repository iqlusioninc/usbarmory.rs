//! Check that we can work with multiple opened files
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example) and format the
//! partition (`emmc-fs-format` example) before running this; otherwise you'll run into a "corrupted
//! filesystem" error

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{consts, transactional, File, FileAlloc, LittleFs, LittleFsAlloc},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILE1: &str = "a.txt";
static FILE2: &str = "b.txt";
static FILE3: &str = "c.txt";
static TESTSTR: &[u8] = b"Hello File!";

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mut mbr = MbrDevice::open(emmc).unwrap();
    let mut main_part = mbr.partition(0).unwrap();

    let mut fsa = LittleFsAlloc::default();
    let fs = LittleFs::mount(&mut fsa, &mut main_part).unwrap();
    memlog!("fs mounted");

    let mut f1_ = FileAlloc::new();
    let mut f2_ = FileAlloc::new();
    let mut f3_ = FileAlloc::new();

    // we need to create the files before we enter transactional mode
    File::create(&fs, &mut f1_, FILE1).unwrap();
    memlog!("created f1");
    File::create(&fs, &mut f2_, FILE2).unwrap();
    memlog!("created f2");
    File::create(&fs, &mut f3_, FILE3).unwrap();
    memlog!("created f3");

    // allows up to 3 open files
    let fs = fs.transactional::<consts::U3>();
    memlog!("entered transactional mode");

    let f1 = transactional::File::open(&fs, &mut f1_, FILE1).unwrap();
    f1.write(TESTSTR).unwrap();
    memlog!("wrote data to file1");

    let f2 = transactional::File::open(&fs, &mut f2_, FILE2).unwrap();
    f2.write(TESTSTR).unwrap();
    memlog!("wrote data to file2");

    let f3 = transactional::File::open(&fs, &mut f3_, FILE3).unwrap();
    f3.write(TESTSTR).unwrap();
    memlog!("wrote data to file3");

    let fs = fs.sync().unwrap();
    memlog!("synced all files -- exiting transactional mode");

    let f1 = File::open(&fs, &mut f1_, FILE1).unwrap();
    let f2 = File::open(&fs, &mut f2_, FILE2).unwrap();
    let f3 = File::open(&fs, &mut f3_, FILE3).unwrap();

    let mut buf = [0; 32];
    for f in [f1, f2, f3].iter_mut() {
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

    memlog!("DONE");

    // then reset the board
    memlog_flush_and_reset!();
}
