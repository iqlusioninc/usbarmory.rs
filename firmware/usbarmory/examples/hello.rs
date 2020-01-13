//! Writes "Hello, world!" to the UART2 interface
//!
//! The message will be displayed on the serial interface used for the u-boot
//! console

#![no_main]
#![no_std]

use core::fmt::Write as _;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::serial::Serial;

#[no_mangle]
fn main() -> ! {
    let mut serial = Serial::get();

    writeln!(serial, "Hello, world!").ok();

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
