//! Blinks the Blue LED

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::led;

#[no_mangle]
fn main() -> ! {
    loop {
        led::Blue.on();
        usbarmory::delay(500_000_000);
        led::Blue.off();
        usbarmory::delay(500_000_000);
    }
}
