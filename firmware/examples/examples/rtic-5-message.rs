//! RTFM message passing
//!
//! Message passing is an alternative to explicit shared memory.
//!
//! This examples makes two tasks exchange 100K messages

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

const THRESHOLD: u32 = 100_000;

#[rtic::app]
const APP: () = {
    #[init(spawn = [ping])]
    fn init(cx: init::Context) {
        // messages can be passed along when `spawn`-ing a task
        cx.spawn.ping(0).unwrap();
        //            ^ message sent
    }

    #[task(spawn = [pong])]
    fn ping(cx: ping::Context, i: u32) {
        //                     ^ message received

        if i > THRESHOLD {
            println!("DONE");
            Serial::flush();
            usbarmory::reset();
        }

        cx.spawn.pong(i + 1).unwrap();
    }

    #[task(spawn = [ping])]
    fn pong(cx: pong::Context, i: u32) {
        if i > THRESHOLD {
            println!("DONE");
            Serial::flush();
            usbarmory::reset();
        }

        cx.spawn.ping(i + 1).unwrap();
    }
};
