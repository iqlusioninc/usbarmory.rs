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

use core::fmt::Write;

use exception_reset as _;
use panic_serial as _;
use rac::gic::{gicc, gicd};
use usbarmory::serial::Serial;

// Lowest priority
const P0: u8 = 0b1111_1000;

// Mid priority
const P1: u8 = 0b1111_0000;

// Higher priority
const P2: u8 = 0b1110_1000;

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
    set_priority_mask(P0);

    // unmask IRQ interrupts
    // (FFI calls implicitly include a memory barrier)
    cortex_a::enable_irq();

    if let Some(serial) = Serial::take() {
        writeln!(&serial, "before SGI0").ok();
        serial.release();
    }

    // send a SGI0 to ourselves
    gicd::GICD_SGIR.write_volatile(0b10 << 24);

    if let Some(serial) = Serial::take() {
        writeln!(&serial, "after SGI0").ok();
        serial.release();
    }

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    usbarmory::reset()
}

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
fn SGI0() {
    if let Some(serial) = Serial::take() {
        writeln!(&serial, "in SGI0").ok();
        serial.release();
    }

    // set the priority mask high enough to mask SGI1
    unsafe {
        set_priority_mask(P2);
    }

    if let Some(serial) = Serial::take() {
        writeln!(&serial, "masked").ok();
        serial.release();
    }

    // send a SGI1 to ourselves
    unsafe {
        gicd::GICD_SGIR.write_volatile((0b10 << 24) | 1);
    }

    // restore the priority mask
    unsafe {
        set_priority_mask(P0);
    }

    //~ preempted by SGI1 here

    if let Some(serial) = Serial::take() {
        writeln!(&serial, "unmasked").ok();
        serial.release();
    }
}

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
fn SGI1() {
    if let Some(serial) = Serial::take() {
        writeln!(&serial, "in SGI1").ok();
        serial.release();
    }
}

unsafe fn set_priority_mask(threshold: u8) {
    gicc::GICC_PMR.write_volatile(u32::from(threshold));
    cortex_a::dmb();
}
