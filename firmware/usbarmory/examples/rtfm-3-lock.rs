//! RTFM `lock` API
//!
//! This is a replica of the `lock` example in the RTFM book [1].
//!
//! Expected output:
//!
//! ```
//! A
//! B - shared = 1
//! C
//! D - shared = 2
//! E
//! ```
//!
//! [1]: https://rtfm.rs/0.5/book/en/by-example/resources.html#lock

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

#[rtfm::app]
const APP: () = {
    // The resources abstraction lets you share memory between tasks
    struct Resources {
        // shared between tasks `foo` and `bar`
        #[init(0)] // <- initial value; must be a `const` evaluable expression
        shared: u64,
    }

    #[init(spawn = [foo])]
    fn init(cx: init::Context) {
        cx.spawn.foo().unwrap();
    }

    // low priority task
    #[task(priority = 1, resources = [shared], spawn = [bar, baz])]
    fn foo(cx: foo::Context) {
        let mut resources = cx.resources;
        let spawn = cx.spawn;

        println!("A");

        // the lower priority task requires a critical section to access the data
        resources.shared.lock(|shared| {
            // data can only be modified within this critical section (closure)
            *shared += 1;

            // `bar` will *not* run right now due to the critical section
            // *if* it ran there would be a data race on the `shared` memory
            spawn.bar().unwrap();

            println!("B - shared = {}", *shared);

            // `baz` does not contend for `shared` so it's allowed to run right
            // away
            spawn.baz().unwrap();
        });

        // critical section is over: `baz` can now start

        println!("E");
    }

    // mid priority task
    #[task(priority = 2, resources = [shared])]
    fn bar(cx: bar::Context) {
        // the higher priority task does *not* need a critical section to access
        // the `shared` data
        *cx.resources.shared += 1;

        println!("D - shared = {}", *cx.resources.shared);
    }

    // high priority task
    #[task(priority = 3)]
    fn baz(_cx: baz::Context) {
        // this task cannot access the `shared` resource because it's not listed
        // in its attribute
        println!("C");
    }

    // will run at the end due to priorities
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        Serial::flush();
        usbarmory::reset()
    }
};
