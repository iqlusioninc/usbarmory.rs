// NOTE using `hidapi` instead of `rusb` because the latter fails to claim the device on macOS

use anyhow::bail;
use hidapi::{HidApi, HidDevice};

use crate::{util, PID, VID};

pub struct Device {
    address: u8,
    inner: HidDevice,
}

impl Device {
    pub fn open() -> Result<Option<Self>, anyhow::Error> {
        let api = HidApi::new()?;

        if let Ok(dev) = api.open(VID, PID) {
            let address = util::get_usb_address()?;
            return Ok(Some(Device {
                inner: dev,
                address,
            }));
        } else {
            Ok(None)
        }
    }

    pub fn address(&self) -> u8 {
        self.address
    }

    pub fn read<'b>(&mut self, buf: &'b mut [u8]) -> Result<&'b [u8], anyhow::Error> {
        let n = self
            .inner
            .read_timeout(buf, util::default_timeout().subsec_millis() as i32)?;
        if n == 0 {
            bail!("HID read timeout")
        } else {
            Ok(&buf[..n])
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), anyhow::Error> {
        self.inner.write(data)?;
        Ok(())
    }
}
