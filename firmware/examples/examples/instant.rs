//! Real Time Counter
//!
//! Expected output:
//!
//! ```
//! 0ns
//! 5s
//! ```

#![no_main]
#![no_std]

use core::time::Duration;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    println,
    serial::Serial,
    time::{self, Instant},
};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let before = Instant::now();

    time::wait(Duration::from_secs(5));

    let elapsed = before.elapsed();
    println!("{:?}", elapsed);

    Serial::flush();

    usbarmory::reset()
}
