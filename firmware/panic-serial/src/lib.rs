#![no_std]

use core::{fmt::Write, panic::PanicInfo};

use usbarmory::serial::Serial;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_a::disable_irq();

    Serial::borrow_unchecked(|mut serial| {
        // NOTE the leading newline is to *not* append the panic message to some
        // other message (in the case we preempted a `write!` operation or a
        // `write!` operation panicked midway)
        write!(serial, "\n\r----\n\r{}\n\r----\n\r", info).ok();
        Serial::flush();
    });

    usbarmory::memlog_flush_and_reset!();
}
