//! Test the `fs::transaction` API
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example)
//!
//! HEADS UP! This example will format the `littlefs` on the first MBR partition

#![no_main]
#![no_std]

use core::convert::TryInto;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{self, consts, File, Fs},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FN1: &[u8] = b"a.txt\0";
static FN2: &[u8] = b"b.txt\0";
static TEXT: &[u8] = b"some great text";

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC").unwrap();

    let mbr = MbrDevice::open(emmc).unwrap();
    let part = mbr.into_partition(0).unwrap();

    let format = true;
    let f = Fs::mount(part, format).unwrap();
    memlog!("fs mounted");

    let fn1 = FN1.try_into().unwrap();
    let fn2 = FN2.try_into().unwrap();

    for filename in [fn1, fn2].iter() {
        let f = File::create(f, *filename).unwrap();
        memlog!("created {}", filename);
        f.close().unwrap();
        memlog!("file closed");
    }

    let transaction = fs::transaction::<consts::U2, _>(f).unwrap();
    let f1 = transaction.open(fn1).unwrap();
    f1.write(TEXT).unwrap();
    memlog!("wrote to file 1's cache");
    let f2 = transaction.open(fn2).unwrap();
    f2.write(TEXT).unwrap();
    memlog!("wrote to file 2's cache");

    // sanity checks: these operations are not allowed
    assert_eq!(
        fs::create_dir(f, b"foo\0".try_into().unwrap()),
        Err(fs::Error::TransactionInProgress)
    );
    assert_eq!(fs::remove(f, fn1), Err(fs::Error::TransactionInProgress));
    assert_eq!(
        fs::rename(f, fn1, fn2),
        Err(fs::Error::TransactionInProgress)
    );
    assert_eq!(
        File::create(f, fn1).err(),
        Some(fs::Error::TransactionInProgress)
    );
    assert_eq!(
        File::open(f, fn1).err(),
        Some(fs::Error::TransactionInProgress)
    );

    transaction.commit().unwrap();
    memlog!("committed files to disk");

    // check the files' contents
    let mut buf = [0; 32];
    for filename in [fn1, fn2].iter() {
        let mut f = File::open(f, *filename).unwrap();
        assert_eq!(f.len().unwrap(), TEXT.len());
        let n = f.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], TEXT);
        memlog!("contents of {} look OK", filename);
    }

    memlog!("DONE");

    // then reset the board
    memlog_flush_and_reset!();
}
