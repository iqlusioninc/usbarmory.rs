//! Just enough to get the USB device enumerated
//!
//! Press any key in terminal to reboot the device and get back to the u-boot
//! console

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDeviceBuilder, UsbVidPid},
};
use usbarmory::{serial::Serial, usbd::Usbd};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let usbd = Usbd::take().expect("Usbd");
    let serial = Serial::take().expect("Serial");

    let allocator = UsbBusAllocator::new(usbd);
    let mut dev = UsbDeviceBuilder::new(&allocator, UsbVidPid(consts::VID, consts::PID))
        .self_powered(true)
        // FIXME other packet sizes don't work right now; we need to report a STATUS out for each partial IN transfer
        .max_packet_size_0(64)
        .product("Ferris")
        .manufacturer("Rustaceans Inc")
        .build();

    loop {
        dev.poll(&mut []);
        usbarmory::memlog_try_flush();

        if serial.try_read().is_some() {
            usbarmory::memlog_flush_and_reset!();
        }
    }
}
