//! RTFM late resources
//!
//! Expected output:
//!
//! ```
//! init
//! foo
//! OK
//! idle
//! OK
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
    struct Resources {
        // a late resource (`init = ..` is missing), a resource initialized at runtime
        x: i32,
    }

    #[init(spawn = [foo])]
    fn init(cx: init::Context) -> init::LateResources {
        cx.spawn.foo().unwrap();

        println!("init");

        // the initial value of `x` must be selected here
        init::LateResources { x: 1 }
    }

    // the resource is initialized here; after `init` returns and before `idle`
    // or any task is allowed to run

    #[idle(resources = [x])]
    fn idle(mut cx: idle::Context) -> ! {
        println!("idle");

        cx.resources.x.lock(|x| {
            assert_eq!(*x, 1);
        });

        println!("OK");

        Serial::flush();

        // then reset the board to return to the u-boot console
        usbarmory::reset()
    }

    #[task(resources = [x])]
    fn foo(cx: foo::Context) {
        println!("foo");

        assert_eq!(*cx.resources.x, 1);

        println!("OK");
    }
};
