//! Writes "Hello, world!" to the UART2 interface
//!
//! The message will be displayed on the serial interface used for the u-boot
//! console

#![no_main]
#![no_std]

use core::fmt::Write as _;

use panic_halt as _;
use usbarmory::serial::Serial;

#[no_mangle]
fn main() -> ! {
    let mut serial = Serial::get();

    writeln!(serial, "Hello, world!").ok();

    loop {}
}
