// NOTE using this instead of the `hidapi` crate because the latter hangs when dropping an
// `HidDevice`

use core::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

pub struct HidDev {
    address: u8,
    handle: DeviceHandle<GlobalContext>,
    timeout: Duration,
}

// see HID v1.11
impl HidDev {
    pub fn from_handle(mut handle: DeviceHandle<GlobalContext>) -> Result<Self, anyhow::Error> {
        let _ = handle.set_auto_detach_kernel_driver(true);
        handle.claim_interface(0)?;
        let address = handle.device().address();
        Ok(Self {
            address,
            handle,
            timeout: Duration::from_millis(100),
        })
    }

    pub fn address(&self) -> u8 {
        self.address
    }

    pub fn set_report(&self, bytes: &[u8]) -> Result<(), anyhow::Error> {
        const REQUEST_TYPE: u8 = 0b00100001;
        const REQUEST_SET_REPORT: u8 = 0x09;
        const REPORT_TYPE_FEATURE: u16 = 0x03;
        const INTERFACE: u16 = 0;

        let report_id = bytes[0];
        self.handle.write_control(
            REQUEST_TYPE,
            REQUEST_SET_REPORT,
            (REPORT_TYPE_FEATURE << 8) | u16::from(report_id),
            INTERFACE,
            bytes,
            self.timeout,
        )?;
        Ok(())
    }

    pub fn read_interrupt<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], anyhow::Error> {
        const ENDPOINT: u8 = 0x81; // EP1_IN
        let len = self.handle.read_interrupt(ENDPOINT, buf, self.timeout)?;
        Ok(&mut buf[..len])
    }
}
