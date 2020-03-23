//! Test interrupt nesting using Software Generated Interrupts (SGI)
//!
//! Expected output:
//!
//! ``` text
//! before SGI0
//! in SGI0
//! masked
//! in SGI1
//! unmasked
//! after SGI0
//! ```

#![no_main]
#![no_std]

use exception_reset as _;
use pac::{gicc::GICC, gicd::GICD};
use panic_serial as _;
use usbarmory::{println, serial::Serial};

// Lowest priority
const P0: u8 = 0b1111_1000;

// Mid priority
const P1: u8 = 0b1111_0000;

// Higher priority
const P2: u8 = 0b1110_1000;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let gicd = GICD::take().expect("UNREACHABLE");

    // IRQ are still masked; this closure cannot be preempted
    GICC::borrow_unchecked(|gicc| {
        // enable the CPU interface
        gicc.CTLR.write(1);

        // enable the distributor
        gicd.CTLR.write(1);

        // SGI0 = mid priority
        unsafe { gicd.IPRIORITYR.write(0, P1) }

        // SGI1 = highest priority
        unsafe { gicd.IPRIORITYR.write(1, P2) }

        // set priority mask to its lowest value
        unsafe { gicc.PMR.write(u32::from(P0)) }
    });

    // unmask IRQ interrupts
    // (FFI calls implicitly include a memory barrier)
    unsafe { cortex_a::enable_irq() }

    println!("before SGI0");

    // send a SGI0 to ourselves
    gicd.SGIR.write(0b10 << 24);

    println!("after SGI0");

    Serial::flush();

    usbarmory::reset()
}

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
fn SGI0() {
    println!("in SGI0");

    // set the priority mask high enough to mask SGI1
    // NOTE(borrow_unchecked) no context performs a Read-Modify-Write operation
    // on this register
    GICC::borrow_unchecked(|gicc| unsafe {
        gicc.PMR.write(u32::from(P2));
    });

    println!("masked");

    // send a SGI1 to ourselves
    // NOTE(borrow_unchecked) single-instruction store operation on a write-only
    // register
    GICD::borrow_unchecked(|gicd| {
        gicd.SGIR.write((0b10 << 24) | 1);
    });

    // restore the priority mask
    // NOTE(borrow_unchecked) no context performs a Read-Modify-Write operation
    // on this register
    GICC::borrow_unchecked(|gicc| unsafe {
        gicc.PMR.write(u32::from(P0));
    });

    // force `GICC_PMR` to be completed before the following `Serial::take` operation
    // without this SGI1 may interrupt SGI0 *after* the `Serial` interface has
    // been taken resulting in SGI1 failing to take the `Serial` interface
    cortex_a::isb(); //~ preempted by SGI1 here

    println!("unmasked");
}

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
fn SGI1() {
    println!("SGI1");
}
