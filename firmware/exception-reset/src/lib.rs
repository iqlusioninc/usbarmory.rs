//! Unhandled exceptions will reset the SoC

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use usbarmory::serial::Serial;

#[allow(non_snake_case)]
#[no_mangle]
unsafe fn DefaultHandler() -> ! {
    let mut serial = Serial::get();

    serial.write_all(b"\nunhandled exception\n");
    serial.flush();

    usbarmory::reset()
}
