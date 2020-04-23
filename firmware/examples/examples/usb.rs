//! USB bulk transfers
//!
//! The `EchoClass` will set up two bulk endpoints (an IN/OUT pair) to receive
//! bulk messages from the host, reverse the message and send it back

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usb_device::{
    bus::{InterfaceNumber, UsbBus, UsbBusAllocator},
    class::UsbClass,
    descriptor::DescriptorWriter,
    device::{UsbDeviceBuilder, UsbVidPid},
    endpoint::{EndpointAddress, EndpointIn, EndpointOut},
};
use usbarmory::{memlog, serial::Serial, usbd::Usbd};

struct EchoClass<'a, B>
where
    B: UsbBus,
{
    iface: InterfaceNumber,
    ep_bulk_in: EndpointIn<'a, B>,
    ep_bulk_out: EndpointOut<'a, B>,
}

/// Max packet size for bulk transfers to/from High-Speed USB devices
const MAX_PACKET_SIZE: u16 = 512;

impl<'b, B> EchoClass<'b, B>
where
    B: UsbBus,
{
    fn new(alloc: &'b UsbBusAllocator<B>) -> Self {
        Self {
            iface: alloc.interface(),
            ep_bulk_in: alloc.bulk(MAX_PACKET_SIZE),
            ep_bulk_out: alloc.bulk(MAX_PACKET_SIZE),
        }
    }
}

impl<B> UsbClass<B> for EchoClass<'_, B>
where
    B: UsbBus,
{
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface(self.iface, 0xff, 0x00, 0x00)?;
        writer.endpoint(&self.ep_bulk_in)?;
        writer.endpoint(&self.ep_bulk_out)?;

        Ok(())
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        memlog!("endpoint_out(addr={:?})", addr);

        let mut buf = [0; MAX_PACKET_SIZE as usize];

        let n = self.ep_bulk_out.read(&mut buf).unwrap();

        buf[..n].reverse();

        self.ep_bulk_in.write(&buf[..n]).unwrap();
    }
}

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let usbd = Usbd::take().expect("Usbd");
    let serial = Serial::take().expect("Serial");

    let allocator = UsbBusAllocator::new(usbd);
    let mut echo_class = EchoClass::new(&allocator);
    let mut dev = UsbDeviceBuilder::new(&allocator, UsbVidPid(consts::VID, consts::PID))
        .self_powered(true)
        .max_packet_size_0(64)
        .product("Ferris")
        .manufacturer("Rustaceans Inc")
        .build();

    loop {
        dev.poll(&mut [&mut echo_class]);
        usbarmory::memlog_try_flush();

        if serial.try_read().is_some() {
            usbarmory::memlog_flush_and_reset!();
        }
    }
}
