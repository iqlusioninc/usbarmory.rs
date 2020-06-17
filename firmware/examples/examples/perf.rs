//! Measure performance (memory / instruction cache)

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial, time::Instant};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    // wall time of executing around 100M instructions
    let start = Instant::now();
    usbarmory::delay(100_000_000);
    let end = Instant::now();

    println!("{:?}", end - start);
    Serial::flush();

    // then reset the board
    usbarmory::reset()
}
