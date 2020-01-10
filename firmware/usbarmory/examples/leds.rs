#![no_main]
#![no_std]

use panic_halt as _;
use usbarmory::led;

#[no_mangle]
fn main() -> ! {
    led::Blue.on();
    led::White.off();

    loop {}
}
