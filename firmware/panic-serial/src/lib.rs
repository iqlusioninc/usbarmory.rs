#![no_std]

use core::{fmt::Write, panic::PanicInfo};

use usbarmory::serial::Serial;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut serial = Serial::get();
    // NOTE the leading newline is to *not* append the panic message to some
    // other message (in the case we preempted a `write!` operation and then
    // panicked)
    writeln!(serial, "\n{}", info).ok();
    serial.flush();

    usbarmory::reset();
}
