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
use pac::{gicc::GICC, gicd::GICD, uart::UART2};
use panic_serial as _;
use usbarmory::{led::Leds, println};

// Lowest priority
const P0: u8 = 0b1111_1000;

// Higher priority
const P1: u8 = 0b1111_0000;

const UART2_IRQ: u8 = 59;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let gicc = GICC::take().expect("UNREACHABLE");
    let gicd = GICD::take().expect("UNREACHABLE");

    // enable the CPU interface
    gicc.CTLR.write(1);

    // enable the distributor
    gicd.CTLR.write(1);

    // enable UART2 interrupt
    unsafe { gicd.ISENABLER.write(UART2_IRQ / 32, 1 << (UART2_IRQ % 32)) }

    // UART2 = higher priority
    unsafe { gicd.IPRIORITYR.write(UART2_IRQ, P1) }

    // set priority mask to its lowest value
    unsafe { gicc.PMR.write(u32::from(P0)) }

    // enable the transmitter empty interrupt
    UART2::borrow_unchecked(|uart| {
        let old = uart.UCR1.read();
        uart.UCR1.write(old | UART_UCR1_TXMPTYEN);
    });

    // start a serial transmission
    println!("before interrupt");

    // unmask IRQ interrupts
    unsafe { cortex_a::enable_irq() }

    // wait 5 seconds ~ serial transmission will finish during this blocking call
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    usbarmory::reset()
}

/// Transmitter Empty Interrupt Enable
const UART_UCR1_TXMPTYEN: u32 = 1 << 6;

#[allow(non_snake_case)]
#[inline(never)]
#[no_mangle]
extern "C" fn UART2() {
    /// Transmit Buffer FIFO Empty
    const UART_USR2_TXFE: u32 = 1 << 14;

    // NOTE(borrow_unchecked) UART is a high priority context; it won't be
    // preempted so this closure is running in a critical section
    UART2::borrow_unchecked(|uart| {
        // clear the interrupt flag
        uart.USR2.write(UART_USR2_TXFE);

        // disable the transmitter empty interrupt (so we don't enter this
        // interrupt handler again)
        let old = uart.UCR1.read();
        uart.UCR1.write(old & !UART_UCR1_TXMPTYEN);
    });

    let leds = Leds::take().expect("UNREACHABLE");
    leds.blue.on();

    println!("UART2");
}
