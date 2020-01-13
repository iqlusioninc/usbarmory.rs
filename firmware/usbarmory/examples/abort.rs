// Generates a data abort exception to test that overriding exception handlers work

#![no_main]
#![no_std]

use exception_reset as _;
use panic_serial as _;
use usbarmory::serial::Serial;

#[no_mangle]
fn main() -> ! {
    // unaligned memory access = data abort exception
    unsafe {
        // this operation will trigger the `DataAbort` handler defined below
        (1 as *const u16).read_volatile();
    }

    usbarmory::reset()
}

#[allow(non_snake_case)]
#[no_mangle]
fn DataAbort() -> ! {
    let mut serial = Serial::get();
    serial.write_all(b"You've met with a terrible fate, haven't you?\n");
    serial.flush();

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    usbarmory::reset()
}
