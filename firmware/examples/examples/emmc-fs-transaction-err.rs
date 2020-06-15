//! Check that overwriting to a file, in transaction mode, triggers an error
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

    for filename in [fn1].iter() {
        let f = File::create(f, *filename).unwrap();
        memlog!("created {}", filename);
        f.close().unwrap();
        memlog!("file closed");
    }

    let transaction = fs::transaction::<consts::U1, _>(f).unwrap();

    let f1 = transaction.open(fn1).unwrap();
    for _ in 0..100 {
        if let Err(e) = f1.write(TEXT) {
            if e == fs::Error::WriteWhileLocked {
                memlog!("error: attempted to write to disk in transaction mode (as expected)");
                memlog_flush_and_reset!();
            } else {
                panic!("{:?}", e);
            }
        }
    }

    panic!("didn't trigger a write to disk?");
}
