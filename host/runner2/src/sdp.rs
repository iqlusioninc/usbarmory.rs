//! Serial Download Protocol (on top of USB HID)

use core::time::Duration;
use std::thread;

use anyhow::bail;
use arrayref::array_ref;

use crate::hid::HidDev;

pub struct Sdp {
    hid: HidDev,
}

const VID: u16 = 0x15a2; // NXP
const PID: u16 = 0x0080; // I.MX6ULZ in SDP mode

impl Sdp {
    pub fn open() -> Result<Self, anyhow::Error> {
        for _ in 0..3 {
            if let Some(handle) = rusb::open_device_with_vid_pid(VID, PID) {
                return Ok(Sdp {
                    hid: HidDev::from_handle(handle)?,
                });
            }

            thread::sleep(Duration::from_secs(1))
        }

        bail!("timeout waiting for USB device {:04x}:{:04x}", VID, PID)
    }

    pub fn usb_address(&self) -> u8 {
        self.hid.address()
    }

    pub fn reconnected(old_address: u8) -> bool {
        if let Ok(devices) = rusb::devices() {
            for dev in devices.iter() {
                if let Ok(dev_desc) = dev.device_descriptor() {
                    if dev_desc.vendor_id() == VID
                        && dev_desc.product_id() == PID
                        && dev.address() != old_address
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn read_memory(&self, address: u32) -> Result<u32, anyhow::Error> {
        self.hid.set_report(
            &Command::ReadRegister {
                address,
                count: 1,
                format: Format::B32,
            }
            .bytes(),
        )?;

        const SEC_REPORT_ID: u8 = 3;
        const SEC_OPEN: [u8; 4] = [0x56, 0x78, 0x78, 0x56];

        let mut sec = [0; 5];
        let sec = self.hid.read_interrupt(&mut sec)?;

        if sec[0] != SEC_REPORT_ID || sec[1..] != SEC_OPEN[..] {
            bail!("HAB is in the closed state");
        }

        const DATA_REPORT_ID: u8 = 4;

        let mut data = [0; 65];
        let data = self.hid.read_interrupt(&mut data)?;

        if data[0] != DATA_REPORT_ID {
            bail!("unexpected report id");
        }

        Ok(u32::from_le_bytes(*array_ref!(data, 1, 4)))
    }
}

enum Command {
    ErrorStatus,

    /// Reads arbitrary memory, but doesn't seem to work on MMIO registers?
    ReadRegister {
        address: u32,
        format: Format,
        count: u32,
    },
}

#[derive(Clone, Copy)]
enum Format {
    B8 = 0x08,
    B16 = 0x10,
    B32 = 0x20,
}

impl Command {
    fn bytes(&self) -> [u8; 17] {
        let mut bytes = [0; 17];
        match self {
            Command::ErrorStatus => {
                // feature report id
                bytes[0] = 0x01;
                let bytes = &mut bytes[1..];

                // command type
                bytes[0] = 0x05;
                bytes[1] = 0x05;
            }

            Command::ReadRegister {
                address,
                format,
                count,
            } => {
                // feature report id
                bytes[0] = 0x01;

                let bytes = &mut bytes[1..];

                // command type
                bytes[0] = 0x01;
                bytes[1] = 0x01;

                // address
                bytes[2..2 + 4].copy_from_slice(&address.to_be_bytes());

                // format
                bytes[6] = *format as u8;

                // data count
                bytes[7..7 + 4].copy_from_slice(&count.to_be_bytes());
            }
        }
        bytes
    }
}
