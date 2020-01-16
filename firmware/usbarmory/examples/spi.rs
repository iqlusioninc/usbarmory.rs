//! Test Shared Peripheral Interrupt
//!
//! Expected output:
//!
//! ``` text
//! before interrupt
//! UART2
//! ```

#![no_main]
#![no_std]

use exception_reset as _;
use panic_serial as _;
use rac::{
    gic::{gicc, gicd},
    uart,
};
use usbarmory::{led::Blue, println};

// Lowest priority
const P0: u8 = 0b1111_1000;

// Higher priority
const P1: u8 = 0b1111_0000;

const UART2_IRQ: usize = 59;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
unsafe fn main() -> ! {
    // enable the CPU interface
    gicc::GICC_CTLR.write_volatile(1);

    // enable the distributor
    gicd::GICD_CTLR.write_volatile(1);

    // enable UART2 interrupt
    gicd::GICD_ISENABLER
        .offset(1)
        .write_volatile(1 << (UART2_IRQ % 32));

    // UART2 = higher priority
    gicd::GICD_IPRIORITYR.add(UART2_IRQ).write_volatile(P1);

    // set priority mask to its lowest value
    gicc::GICC_PMR.write_volatile(u32::from(P0));

    // enable the transmitter empty interrupt
    let old = uart::UART2_UCR1.read_volatile();
    uart::UART2_UCR1.write_volatile(old | (1 << 6));

    // start a serial transmission
    println!("before interrupt");

    // unmask IRQ interrupts
    cortex_a::enable_irq();

    // wait 5 seconds ~ serial transmission will finish during this blocking call
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    usbarmory::reset()
}

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
unsafe extern "C" fn UART2() {
    // clear the interrupt flag
    uart::UART2_USR2.write_volatile(1 << 14);

    // disable the transmitter empty interrupt (so we don't enter this interrupt handler again)
    let old = uart::UART2_UCR1.read_volatile();
    uart::UART2_UCR1.write_volatile(old & !(1 << 6));

    if let Some(blue) = Blue::take() {
        blue.on();
        blue.release();
    }

    println!("UART2");
}
