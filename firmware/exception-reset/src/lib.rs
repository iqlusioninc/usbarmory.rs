//! Unhandled exceptions will reset the SoC

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use usbarmory::serial::Serial;

#[allow(non_snake_case)]
#[no_mangle]
unsafe fn DefaultHandler() -> ! {
    Serial::borrow_unchecked(|serial| {
        cortex_a::disable_fiq();
        cortex_a::disable_irq();

        // NOTE the leading newline is to *not* append the panic message to some
        // other message (in the case we preempted a `write!` operation)
        serial.write_all(b"\nunhandled exception\n");
        Serial::flush();
    });

    usbarmory::reset()
}
