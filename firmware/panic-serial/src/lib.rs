#![no_std]

use core::{fmt::Write, panic::PanicInfo};

use usbarmory::serial::Serial;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Serial::borrow_unchecked(|mut serial| {
        // NOTE the leading newline is to *not* append the panic message to some
        // other message (in the case we preempted a `write!` operation or a
        // `write!` operation panicked midway)
        writeln!(serial, "\n{}", info).ok();
        Serial::flush();
    });

    usbarmory::reset();
}
