//! Blinks the Blue LED

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::led;

#[no_mangle]
fn main() -> ! {
    if let Some(blue) = led::Blue::take() {
        loop {
            blue.on();
            usbarmory::delay(usbarmory::CPU_FREQUENCY);
            blue.off();
            usbarmory::delay(usbarmory::CPU_FREQUENCY);
        }
    } else {
        usbarmory::reset()
    }
}
