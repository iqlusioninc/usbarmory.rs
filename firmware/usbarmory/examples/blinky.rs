#![no_main]
#![no_std]

use panic_halt as _;
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
