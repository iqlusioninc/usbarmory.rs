use core::time::Duration;

use anyhow::bail;

use crate::{PID, VID};

#[allow(dead_code)]
pub(crate) fn get_usb_address() -> Result<u8, anyhow::Error> {
    for dev in rusb::devices()?.iter() {
        let desc = dev.device_descriptor()?;

        if desc.vendor_id() == VID && desc.product_id() == PID {
            return Ok(dev.address());
        }
    }

    bail!("device {:04x}:{:04x} not found", VID, PID)
}

/// Checks if the USB serial downloader has been re-enumerated
pub fn has_been_reenumerated(last_address: u8) -> bool {
    if let Ok(devices) = rusb::devices() {
        for dev in devices.iter() {
            if let Ok(dev_desc) = dev.device_descriptor() {
                if dev_desc.vendor_id() == VID
                    && dev_desc.product_id() == PID
                    && dev.address() != last_address
                {
                    return true;
                }
            }
        }
    }

    false
}

pub fn default_timeout() -> Duration {
    Duration::from_millis(100)
}
