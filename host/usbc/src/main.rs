//! Sample USB client
//!
//! `usbc some message` will send the command line arguments to the device using 
//! bulk transfers. The program will then block wait for response bulk
//! transfers (one for each transfer sent) and then prints their contents to the
//! console, one line per response transfer.

use core::time::Duration;
use std::{env, str};

use anyhow::bail;
use rusb::{DeviceHandle, Direction, GlobalContext, TransferType};

#[derive(Clone, Copy, Debug)]
struct Endpoint {
    address: u8,
    config: u8,
    iface: u8,
    setting: u8,
}

// NOTE assuming a High-Speed USB device
const MAX_PACKET_SIZE: usize = 512;

fn main() -> Result<(), anyhow::Error> {
    let mut bulk = BulkPair::open(consts::VID, consts::PID)?;

    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        bail!("expected at least one argument")
    }

    for msg in args {
        assert!(
            msg.len() < MAX_PACKET_SIZE,
            "messages longer than the maximum packet size are currently not supported"
        );

        bulk.write(msg.as_bytes())?;
        let mut buf = [0; MAX_PACKET_SIZE];
        let resp = bulk.read(&mut buf)?;

        if let Ok(s) = str::from_utf8(resp) {
            println!("{}", s)
        } else {
            println!("{:?}", resp)
        }
    }

    Ok(())
}

fn default_timeout() -> Duration {
    Duration::from_millis(100)
}

/// IN/OUT bulk endpoint pair
struct BulkPair {
    ep_addr_in: u8,
    ep_addr_out: u8,
    handle: DeviceHandle<GlobalContext>,
    timeout: Duration,
}

impl BulkPair {
    pub fn open(vid: u16, pid: u16) -> Result<Self, anyhow::Error> {
        for dev in rusb::devices()?.iter() {
            let desc = dev.device_descriptor()?;

            if desc.vendor_id() == vid && desc.product_id() == pid {
                for i in 0..desc.num_configurations() {
                    let config_desc = dev.config_descriptor(i)?;

                    for iface in config_desc.interfaces() {
                        'iface: for iface_desc in iface.descriptors() {
                            let mut ep_addr_in = None;
                            let mut ep_addr_out = None;
                            let mut seen = 0;

                            for ep_desc in iface_desc.endpoint_descriptors() {
                                if ep_desc.transfer_type() == TransferType::Bulk {
                                    seen += 1;
                                    let addr = ep_desc.address();
                                    if ep_desc.direction() == Direction::In {
                                        ep_addr_in = Some(addr);
                                    } else {
                                        ep_addr_out = Some(addr);
                                    };
                                }
                            }

                            if let (Some(ep_addr_in), Some(ep_addr_out)) = (ep_addr_in, ep_addr_out)
                            {
                                if seen != 2 {
                                    // more than one IN/OUT endpoint pair; try the next interface
                                    continue 'iface;
                                }

                                let mut handle = dev.open()?;
                                handle.set_auto_detach_kernel_driver(true).or_else(|err| {
                                    if err == rusb::Error::NotSupported {
                                        Ok(())
                                    } else {
                                        Err(err)
                                    }
                                })?;

                                let iface_nr = iface_desc.interface_number();
                                let active_config = handle.active_configuration()?;
                                let config_nr = config_desc.number();
                                if config_nr != active_config {
                                    handle.set_active_configuration(config_nr)?;
                                }

                                handle.claim_interface(iface_nr)?;

                                handle
                                    .set_alternate_setting(iface_nr, iface_desc.setting_number())?;

                                return Ok(BulkPair {
                                    handle,
                                    ep_addr_in,
                                    ep_addr_out,
                                    timeout: default_timeout(),
                                });
                            }
                        }
                    }
                }

                bail!("found matching device but not a matching interface");
            }
        }

        bail!("USB device {:04x}:{:04x} not found", vid, pid);
    }

    /// Reads data from the IN endpoint
    pub fn read<'b>(&mut self, buf: &'b mut [u8]) -> Result<&'b [u8], anyhow::Error> {
        let n = self.handle.read_bulk(self.ep_addr_in, buf, self.timeout)?;
        Ok(&buf[..n])
    }

    /// Writes data into the OUT endpoint
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), anyhow::Error> {
        self.handle
            .write_bulk(self.ep_addr_out, bytes, self.timeout)?;
        Ok(())
    }

    /// Sets a timeout for bulk transfer
    ///
    /// The default is twice the interval between USB frames, so 2 milliseconds
    #[allow(dead_code)]
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}
