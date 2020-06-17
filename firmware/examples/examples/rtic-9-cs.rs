//! Test the critical section API
//!
//! Expected output:
//!
//! ```
//! before critical section
//! before spawn
//! after spawn
//! foo
//! after critical section
//! ```

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

#[rtic::app]
const APP: () = {
    // this is a safe version of the `no_mangle` main that the other examples use
    #[idle(spawn = [foo])]
    fn idle(cx: idle::Context) -> ! {
        println!("before critical section");

        // no task or interrupt can preempt this closure
        usbarmory::no_interrupts(|| {
            println!("before spawn");
            cx.spawn.foo().unwrap();
            println!("after spawn");
        });

        println!("after critical section");

        Serial::flush();

        // then reset the board to return to the u-boot console
        usbarmory::reset()
    }

    #[task]
    fn foo(_cx: foo::Context) {
        println!("foo");
    }
};
