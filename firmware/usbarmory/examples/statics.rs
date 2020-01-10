//! Sanity check that static variables work

#![no_main]
#![no_std]

use core::{
    fmt::Write,
    sync::atomic::{AtomicU64, Ordering},
};

use panic_halt as _;
use usbarmory::serial::Serial;

// .bss
static X: AtomicU64 = AtomicU64::new(0);
// .data
static Y: AtomicU64 = AtomicU64::new(1);

#[no_mangle]
fn main() -> ! {
    let mut serial = Serial::get();
    writeln!(
        serial,
        "X={}, Y={}",
        X.load(Ordering::Relaxed),
        Y.load(Ordering::Relaxed)
    )
    .ok();
    X.fetch_add(1, Ordering::Relaxed);
    Y.fetch_add(1, Ordering::Relaxed);
    writeln!(
        serial,
        "X={}, Y={}",
        X.load(Ordering::Relaxed),
        Y.load(Ordering::Relaxed)
    )
    .ok();

    loop {}
}
