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
use panic_serial as _;
use rac::gic::{gicc, gicd};
use usbarmory::println;

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
unsafe fn main() -> ! {
    // enable the CPU interface
    gicc::GICC_CTLR.write_volatile(1);

    // enable the distributor
    gicd::GICD_CTLR.write_volatile(1);

    // SGI0 = mid priority
    gicd::GICD_IPRIORITYR.write_volatile(P1);

    // SGI1 = highest priority
    gicd::GICD_IPRIORITYR.add(1).write_volatile(P2);

    // set priority mask to its lowest value
    gicc::GICC_PMR.write_volatile(u32::from(P0));

    // unmask IRQ interrupts
    // (FFI calls implicitly include a memory barrier)
    cortex_a::enable_irq();

    println!("before SGI0");

    // send a SGI0 to ourselves
    gicd::GICD_SGIR.write_volatile(0b10 << 24);

    println!("after SGI0");

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    usbarmory::reset()
}

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
fn SGI0() {
    println!("in SGI0");

    // set the priority mask high enough to mask SGI1
    unsafe {
        gicc::GICC_PMR.write_volatile(u32::from(P2));
    }

    println!("masked");

    // send a SGI1 to ourselves
    unsafe {
        gicd::GICD_SGIR.write_volatile((0b10 << 24) | 1);
    }

    // restore the priority mask
    unsafe {
        gicc::GICC_PMR.write_volatile(u32::from(P0));
    }

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
