//! Blinks the Blue LED

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::led;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
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
