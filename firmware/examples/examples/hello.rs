//! Writes "Hello, world!" to the UART2 interface
//!
//! The message will be displayed on the serial interface used for the u-boot
//! console

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    println!("Hello, world!");

    Serial::flush();

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
