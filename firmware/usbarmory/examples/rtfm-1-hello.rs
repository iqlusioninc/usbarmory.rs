//! "Hello, world!" using RTFM

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::println;

#[rtfm::app]
const APP: () = {
    // this is a safe version of the `no_mangle` main that the other examples use
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        println!("Hello, world!");

        // wait 5 seconds
        usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

        // then reset the board to return to the u-boot console
        usbarmory::reset()
    }
};
