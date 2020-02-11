//! Real Time Counter
//!
//! Expected output:
//!
//! ```
//! 0ns
//! 5.000064849s
//! ```

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, time::Instant, serial::Serial};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let before = Instant::now();

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    let elapsed = before.elapsed();
    println!("{:?}", elapsed);

    Serial::flush();

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
