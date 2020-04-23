//! Blinks the Blue LED

#![no_main]
#![no_std]

use core::time::Duration;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{led::Leds, serial::Serial, time};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let leds = Leds::take().expect("UNREACHABLE");
    let serial = Serial::take().expect("UNREACHABLE");
    let dur = Duration::from_secs(1);

    loop {
        leds.blue.on();
        time::wait(dur);
        leds.blue.off();
        time::wait(dur);

        // reboot the system if the user pressed a key
        // NOTE as the Cargo runner does NOT currently read stdin you'll need to power cycle the
        // board to flash a new program
        if serial.try_read().is_some() {
            usbarmory::reset()
        }
    }
}
