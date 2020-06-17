//! Turns the blue LED on and the white LED off

#![no_main]
#![no_std]

use core::time::Duration;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{led::Leds, time};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let leds = Leds::take().expect("UNREACHABLE");
    leds.blue.on();
    leds.white.off();

    // wait 5 seconds
    time::wait(Duration::from_secs(5));

    // then reset the board
    usbarmory::reset()
}
