// NOTE using `rusb` instead of `hidapi` because the latter blocks for several seconds when
// you drop an `HidDevice` (at least on Linux)

use core::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

use crate::{util, PID, VID};

pub struct Device {
    address: u8,
    handle: DeviceHandle<GlobalContext>,
    timeout: Duration,
}

impl Device {
    pub fn open() -> Result<Option<Self>, anyhow::Error> {
        if let Some(mut handle) = rusb::open_device_with_vid_pid(VID, PID) {
            let _ = handle.set_auto_detach_kernel_driver(true);
            handle.claim_interface(0)?;
            let address = handle.device().address();
            Ok(Some(Self {
                address,
                handle,
                timeout: util::default_timeout(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Returns the address of this HID device
    pub fn address(&self) -> u8 {
        self.address
    }

    /// Sends a
    // see HID v1.11 spec -- SET_REPORT request over control endpoint 0
    pub fn write(&self, report: &[u8]) -> Result<(), anyhow::Error> {
        const REQUEST_TYPE: u8 = 0b00100001;
        const REQUEST_SET_REPORT: u8 = 0x09;
        const REPORT_TYPE_FEATURE: u16 = 0x03;
        const INTERFACE: u16 = 0;

        let report_id = report[0];
        self.handle.write_control(
            REQUEST_TYPE,
            REQUEST_SET_REPORT,
            (REPORT_TYPE_FEATURE << 8) | u16::from(report_id),
            INTERFACE,
            report,
            self.timeout,
        )?;
        Ok(())
    }

    pub fn read<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], anyhow::Error> {
        // FIXME the correct thing to do would be to get the endpoint address from the interface (?)
        // descriptor when we `open` the device
        const ENDPOINT: u8 = 0x81; // EP1_IN
        let len = self.handle.read_interrupt(ENDPOINT, buf, self.timeout)?;
        Ok(&mut buf[..len])
    }
}
