//! Writes "Hello, world!" the UART2 interface
//!
//! This will display on the serial interface where you can see the u-boot
//! console

#![no_main]
#![no_std]

use core::fmt::Write as _;

use panic_halt as _;
use usbarmory::Serial;

#[no_mangle]
fn main() -> ! {
    let mut serial = Serial::get();

    writeln!(serial, "Hello, world!").ok();

    loop {}
}
