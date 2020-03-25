//! RTFM task local variables

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use heapless::{
    pool,
    pool::singleton::{Box, Pool},
};
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

// A memory pool, usable from any context
pool!(P: [u8; 128]);

const THRESHOLD: usize = 1_000;

#[rtfm::app]
const APP: () = {
    #[init(spawn = [ping])]
    fn init(cx: init::Context) {
        // A task local variable
        static mut MEMORY: [u8; 1024] = [0; 1024];

        // the framework transform the above static variable into a `&'static
        // mut` pointer (a pseudo `Box`; the pointer also has move semantics)
        let memory: &'static mut [u8; 1024] = MEMORY;

        // increase the capacity of the pool to around 8 `[u8; 128]` buffers
        P::grow(memory);

        // get a unused buffer from the pool
        let mut x = P::alloc().expect("OOM").freeze();
        // initialize its contents
        x.iter_mut().for_each(|x| *x = 0);

        cx.spawn.ping(x).ok().expect("UNREACHABLE");
    }

    #[task(spawn = [pong])]
    fn ping(cx: ping::Context, mut x: Box<P>) {
        // A task local variable
        // this static variable is safe to access and can be used to persist
        // state across different runs of this task
        static mut STATE: usize = 0;

        // in tasks static variables are transformed into *non* static `mut`
        // references
        let state: &mut usize = STATE;

        // "verify" received message
        x.iter().for_each(|x| assert_eq!(*x, *state as u8));
        *state += 1;

        if *state > THRESHOLD {
            println!("DONE");
            Serial::flush();
            usbarmory::reset();
        }

        // reuse the buffer to send a new message
        x.iter_mut().for_each(|x| *x = *state as u8);

        cx.spawn.pong(x).ok().expect("UNREACHABLE");
    }

    #[task(spawn = [ping])]
    fn pong(cx: pong::Context, x: Box<P>) {
        // get a new free buffer from the pool
        let mut y = P::alloc().expect("OOM").freeze();

        // do some work
        y.copy_from_slice(&*x);

        // this happens automatically but we are explicit here to explain what happens on `drop`
        drop(x); // this returns the buffer to the pool

        cx.spawn.ping(y).ok().expect("UNREACHABLE");
    }
};
