//! Check that littlefs2 behaves sanely in an edge case
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
    fs::{File, Fs, SeekFrom},
    memlog, memlog_flush_and_reset,
    storage::MbrDevice,
};

static FILENAME: &[u8] = b"hello.txt\0";
static TEXT: &[u8] = b"Hello File!";

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
    let mut f1 = File::create(f, filename).unwrap();
    memlog!("created {}", filename);
    let n = f1.write(TEXT).unwrap();
    f1.close().unwrap();
    memlog!("wrote {}B to file", n);

    let mut f1 = File::open(f, filename).unwrap();
    memlog!("opened {}", filename);

    let off = f1.seek(SeekFrom::Start(4)).unwrap();
    memlog!("moved cursor to byte {}", off);

    let mut buf = [0; 32];
    let n = f1.read(&mut buf).unwrap();

    let off = off as usize;
    assert_eq!(&buf[..n], &TEXT[off..]);

    memlog!("file contents look OK");
    memlog!("DONE");

    // then reset the board
    memlog_flush_and_reset!()
}
