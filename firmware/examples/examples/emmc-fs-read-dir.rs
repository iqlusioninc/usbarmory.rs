//! Check that littlefs2 behaves sanely in an edge case
//!
//! NOTE if you haven't already create an MBR partition (`emmc-new-mbr` example)
//!
//! HEADS UP this example will format the `littlefs` on the first MBR partition

#![no_main]
#![no_std]

use core::convert::TryInto;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{self, File, Fs, Path},
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

    let format = true;
    let f = Fs::mount(part, format).unwrap();
    memlog!("fs mounted");

    fs::create_dir(f, b"foo\0".try_into().unwrap()).unwrap();
    fs::create_dir(f, b"foo/bar\0".try_into().unwrap()).unwrap();
    fs::create_dir(f, b"baz\0".try_into().unwrap()).unwrap();

    File::create(f, b"/foo/bar/quux.txt\0".try_into().unwrap())
        .unwrap()
        .write(b"Hello")
        .unwrap();

    recurse(f, b"/\0".try_into().unwrap());

    // then reset the board
    memlog_flush_and_reset!()
}

fn recurse(f: Fs, path: &Path) {
    for entry in fs::read_dir(f, path).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name();

        if filename != "." && filename != ".." {
            memlog!("{:?} @ {}", entry, path);

            if entry.file_type().is_dir() {
                let path = path.join(filename);
                recurse(f, &path);
            }
        }
    }
}
