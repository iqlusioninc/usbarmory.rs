//! RTFM `spawn` API
//!
//! Used to start (software) tasks in software
//!
//! Expected output:
//!
//! ```
//! before spawn
//! after spawn
//! foo
//! idle
//! ```

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

#[rtfm::app]
const APP: () = {
    // the initialization function
    //
    // this is the first function that runs
    #[init(spawn = [foo])]
    fn init(cx: init::Context) {
        println!("before spawn");

        // start task `foo`
        // note that, from this `Context`, you can only spawn tasks declared in
        // the `spawn` list (see the `#[init]` attribute)
        cx.spawn.foo().expect("UNREACHABLE");

        println!("after spawn");
    }

    // only after `init` returns can tasks and `idle` start their execution

    // a software task
    //
    // unless specified otherwise tasks have a default priority of 1
    #[task]
    fn foo(_cx: foo::Context) {
        println!("foo");
    }

    // the idle context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        // this code only runs when no task is running because it runs at the lowest priority of `0`
        println!("idle");

        Serial::flush();
        usbarmory::reset()
    }
};
