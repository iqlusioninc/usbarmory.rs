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
use usbarmory::{println, rtc::Rtc, serial::Serial};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let rtc = Rtc::initialize().expect("UNREACHABLE");

    let now = rtc.elapsed();
    println!("{:?}", now);

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    let then = rtc.elapsed();
    println!("{:?}", then);

    Serial::flush();

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
