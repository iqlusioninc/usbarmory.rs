//! Testing the `debug_timebox` API
//!
//! Expected output:
//!
//! - `dev`
//!
//! ```
//! doing some hard work
//! panicked at 'work was not completed within 1µs', <::core::macros::panic macros>:5:50
//! ```
//!
//! - `release`
//!
//! ```
//! doing some hard work
//! The answer was 42
//! ```

#![no_main]
#![no_std]

use core::time::Duration;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    // The serial interface is slow so this won't complete within one microsecond
    // This will panic when compiling with the `dev` profile but pass when
    // compiling with the `release` profile
    let ans = usbarmory::debug_timebox(Duration::from_micros(1), || {
        println!("doing some hard work");
        42
    });

    println!("The answer was {}", ans);

    Serial::flush();

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
