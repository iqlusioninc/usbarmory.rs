//! Turns the blue LED on and the white LED off

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::led;

#[no_mangle]
fn main() -> ! {
    led::Blue.on();
    led::White.off();

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
