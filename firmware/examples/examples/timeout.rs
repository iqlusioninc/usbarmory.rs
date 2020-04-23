//! Timeout pattern

#![no_main]
#![no_std]

use core::time::Duration;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial, time::Instant};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let timeout = Duration::from_millis(10);

    let now = Instant::now();
    while !status_flag_is_set() {
        if now.elapsed() > timeout {
            println!("timed out");
            break;
        }
    }

    Serial::flush();

    // then reset the board
    usbarmory::reset()
}

fn status_flag_is_set() -> bool {
    false
}
