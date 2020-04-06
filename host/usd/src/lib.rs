//! USB Serial Downloader for the i.MX6UL(Z)
//!
//! # References
//!
//! - section 8.9.3.1 "SDP commands" of the i.MX 6ULZ Applications Processor Reference Manual
//!   (IMX6ULZRM)

use core::{
    ops::{self, Range},
    time::Duration,
};
use std::{
    io::{self, Write as _},
    thread,
};

use anyhow::bail;
use arrayref::array_ref;

/// HID backend
#[cfg_attr(feature = "rusb-hid", path = "hid/rusb.rs")]
#[cfg_attr(feature = "hidapi", path = "hid/hidapi.rs")]
pub mod hid;
pub mod util;

/// USB Serial Downloader
///
/// This abstraction implements the i.MX Serial Downloader Protocol (SDP)
pub struct Usd {
    hid: hid::Device,
    verbose: bool,
}

pub const VID: u16 = 0x15a2; // NXP
pub const PID: u16 = 0x0080; // I.MX6ULZ in Serial Downloader (SD) mode
pub const OCRAM_FREE_ADDRESS: u32 = 0x91_0000;

impl Usd {
    /// Opens the USB Serial Downloader device
    pub fn open() -> Result<Self, anyhow::Error> {
        for _ in 0..3 {
            if let Some(hid) = hid::Device::open()? {
                return Ok(Usd {
                    hid,
                    verbose: false,
                });
            }

            thread::sleep(Duration::from_secs(1))
        }

        bail!("timeout waiting for USB device {:04x}:{:04x}", VID, PID)
    }

    /// Changes the verbosity of the API
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Returns the USB address of the USB Serial Downloader device
    pub fn usb_address(&self) -> u8 {
        self.hid.address()
    }

    /// Writes the given `dcd` into the device's memory at `address`
    pub fn dcd_write(&mut self, address: u32, dcd: &[u8]) -> Result<(), anyhow::Error> {
        if self.verbose {
            eprintln!("dcd_write(address={:#010x}, count={})", address, dcd.len());
        }

        assert!(dcd.len() < u32::max_value() as usize);

        self.send_command(Command::DcdWrite {
            address,
            count: dcd.len() as u32,
        })?;

        const DATA_REPORT_ID: u8 = 2;
        const DATA_REPORT_SIZE: usize = 1024;

        let mut report = [0; DATA_REPORT_SIZE + 1];
        report[0] = DATA_REPORT_ID;
        for chunk in dcd.chunks(DATA_REPORT_SIZE) {
            let n = chunk.len();
            if self.verbose {
                eprint!(".");
                io::stdout().flush()?;
            }
            report[1..n + 1].copy_from_slice(chunk);

            self.hid.write(&report[..n + 1])?;
        }
        if self.verbose {
            eprintln!();
        }

        self.check_sec_status()?;

        const RESP_WRITE_COMPLETE: [u8; 4] = [0x12, 0x8A, 0x8A, 0x12];
        let resp = self.read_response()?;
        if !resp.starts_with(&RESP_WRITE_COMPLETE) {
            bail!(
                "device didn't acknowledge the DCD_WRITE command (resp={:?})",
                &*resp
            )
        }

        if self.verbose {
            eprintln!("DCD_WRITE ACK-ed");
        }

        Ok(())
    }

    /// Sends the JUMP_ADDRESS command
    ///
    /// `address` must be the address of a previously written DCD
    ///
    /// The device won't respond to further commands
    pub fn jump_address(mut self, address: u32) -> Result<(), anyhow::Error> {
        if self.verbose {
            eprintln!("jump_address(address={:#010x})", address);
        }

        self.send_command(Command::JumpAddress { address })?;

        self.check_sec_status()?;

        if self.verbose {
            eprintln!("JUMP_ADDRESS ACK-ed");
        }

        // the USB device is disconnected at this point; forget the handle to avoid trying to
        // communicate with it
        core::mem::forget(self);

        Ok(())
    }

    /// Reads one word of device memory
    pub fn read_memory(&mut self, address: u32) -> Result<u32, anyhow::Error> {
        assert_eq!(address % 4, 0, "address must be 4-byte aligned");

        self.send_command(Command::ReadRegister {
            address,
            count: 1,
            format: Format::B32,
        })?;

        self.check_sec_status()?;

        let resp = self.read_response()?;

        Ok(u32::from_le_bytes(*array_ref!(resp, 1, 4)))
    }

    #[cfg(untested)]
    pub fn skip_dcd_header(&mut self) -> Result<(), anyhow::Error> {
        if self.verbose {
            eprintln!("skip_dcd_header");
        }

        self.send_command(Command::SkipDcdHeader)?;

        self.check_sec_status()?;

        // this is what the docs say
        // const RESP_ACK_OK: [u8; 4] = [0x90, 0x0D, 0xD0, 0x09];
        // this is what the device actually answers with
        const RESP_ACK_OK: [u8; 4] = [0x09, 0xD0, 0x0D, 0x90];
        let resp = self.read_response()?;
        if !resp.starts_with(&RESP_ACK_OK) {
            bail!(
                "device didn't acknowledge the SKIP_DCD_HEADER command (resp={:?})",
                &*resp
            )
        }

        if self.verbose {
            eprintln!("SKIP_DCD_HEADER ACK-ed");
        }

        Ok(())
    }

    /// Writes the given `data` into the device's memory at `address`
    pub fn write_file(&mut self, address: u32, data: &[u8]) -> Result<(), anyhow::Error> {
        if self.verbose {
            eprintln!(
                "write_file(address={:#010x}, count={})",
                address,
                data.len()
            );
        }

        assert!(data.len() < u32::max_value() as usize);

        self.send_command(Command::WriteFile {
            address,
            count: data.len() as u32,
        })?;

        const DATA_REPORT_ID: u8 = 2;
        const DATA_REPORT_SIZE: usize = 1024;

        let mut report = [0; DATA_REPORT_SIZE + 1];
        report[0] = DATA_REPORT_ID;
        for chunk in data.chunks(DATA_REPORT_SIZE) {
            let n = chunk.len();
            if self.verbose {
                eprint!(".");
                io::stdout().flush()?;
            }
            report[1..n + 1].copy_from_slice(chunk);

            self.hid.write(&report[..n + 1])?;
        }
        if self.verbose {
            eprintln!();
        }

        self.check_sec_status()?;

        const RESP_COMPLETE: [u8; 4] = [0x88, 0x88, 0x88, 0x88];
        let resp = self.read_response()?;
        if !resp.starts_with(&RESP_COMPLETE) {
            bail!(
                "device didn't acknowledge the WRITE_FILE command (resp={:?})",
                &*resp
            )
        }

        if self.verbose {
            eprintln!("WRITE_FILE ACK-ed");
        }

        Ok(())
    }

    #[cfg(untested)]
    pub fn write_memory(&mut self, address: u32, data: Data) -> Result<(), anyhow::Error> {
        assert_eq!(address % 4, 0, "address must be 4-byte aligned");

        if self.verbose {
            eprintln!("write_register(address={:#010x}, data={:?})", address, data);
        }

        self.send_command(Command::WriteRegister { address, data })?;

        self.check_sec_status()?;

        const RESP_WRITE_COMPLETE: [u8; 4] = [0x12, 0x8A, 0x8A, 0x12];
        let resp = self.read_response()?;
        if !resp.starts_with(&RESP_WRITE_COMPLETE) {
            bail!(
                "device didn't acknowledge the WRITE_REGISTER command (resp={:?})",
                &*resp
            )
        }

        if self.verbose {
            eprintln!("WRITE_REGISTER ACK-ed");
        }

        Ok(())
    }

    fn send_command(&mut self, cmd: Command) -> Result<(), anyhow::Error> {
        self.hid.write(&cmd.bytes())?;
        Ok(())
    }

    fn check_sec_status(&mut self) -> Result<(), anyhow::Error> {
        const SEC_REPORT_ID: u8 = 3;
        const SEC_OPEN: [u8; 4] = [0x56, 0x78, 0x78, 0x56];

        let mut buf = [0; 5];
        let mut sec = self.hid.read(&mut buf)?;
        // HACK(hidapi) no idea why sometimes we get a 1 byte read here but doing another read
        // returns the right data
        if sec.len() == 1 {
            sec = self.hid.read(&mut buf)?;
        }

        if sec[0] != SEC_REPORT_ID || sec[1..] != SEC_OPEN[..] {
            bail!("HAB is in the closed state ({:?})", sec);
        }

        Ok(())
    }

    fn read_response<'b>(&mut self) -> Result<Response, anyhow::Error> {
        const RESPONSE_REPORT_ID: u8 = 4;

        let mut resp = Response::default();
        let report = self.hid.read(&mut resp.buffer)?;

        if report[0] != RESPONSE_REPORT_ID {
            bail!("unexpected report id");
        }

        resp.len = report.len();

        Ok(resp)
    }
}

struct Response {
    buffer: [u8; 65],
    len: usize,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            buffer: [0; 65],
            len: 0,
        }
    }
}

impl ops::Deref for Response {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.buffer[1..self.len]
    }
}

/// SDP commands
#[allow(dead_code)]
enum Command {
    DcdWrite {
        address: u32,
        count: u32,
    },

    ErrorStatus,

    JumpAddress {
        address: u32,
    },

    /// Reads an arbitrary memory location, but doesn't seem to work on MMIO registers?
    ReadRegister {
        address: u32,
        format: Format,
        count: u32,
    },

    SkipDcdHeader,

    WriteFile {
        address: u32,
        count: u32,
    },

    /// Writes to an arbitrary memory location
    WriteRegister {
        address: u32,
        data: Data,
    },
}

#[derive(Clone, Copy)]
enum Format {
    B8 = 0x08,
    B16 = 0x10,
    B32 = 0x20,
}

#[derive(Clone, Copy, Debug)]
pub enum Data {
    U8(u8),
    U16(u16),
    U32(u32),
}

impl Command {
    fn bytes(&self) -> [u8; 17] {
        const REPORT_ID: usize = 0;
        const CMD_TY1: usize = 0;
        const CMD_TY2: usize = 1;
        const ADDRESS: Range<usize> = 2..2 + 4;
        const FORMAT: usize = ADDRESS.end;
        const DATA_COUNT: Range<usize> = FORMAT + 1..FORMAT + 5;
        const DATA: Range<usize> = DATA_COUNT.end..DATA_COUNT.end + 4;

        let mut bytes = [0; 17];
        // feature report id
        bytes[REPORT_ID] = 1;
        let cmd = &mut bytes[REPORT_ID + 1..];
        let cmdty = match self {
            Command::DcdWrite { address, count } | Command::WriteFile { address, count } => {
                cmd[ADDRESS].copy_from_slice(&address.to_be_bytes());
                cmd[DATA_COUNT].copy_from_slice(&count.to_be_bytes());

                match self {
                    Command::DcdWrite { .. } => 0x0A,
                    Command::WriteFile { .. } => 0x04,
                    _ => unreachable!(),
                }
            }

            Command::ErrorStatus => 0x05,

            Command::JumpAddress { address } => {
                cmd[ADDRESS].copy_from_slice(&address.to_be_bytes());

                0x0b
            }

            Command::ReadRegister {
                address,
                format,
                count,
            } => {
                cmd[ADDRESS].copy_from_slice(&address.to_be_bytes());
                cmd[FORMAT] = *format as u8;
                cmd[DATA_COUNT].copy_from_slice(&count.to_be_bytes());

                0x01
            }

            Command::SkipDcdHeader => 0x0C,

            Command::WriteRegister { address, data } => {
                cmd[ADDRESS].copy_from_slice(&address.to_be_bytes());
                match data {
                    Data::U8(byte) => {
                        cmd[FORMAT] = Format::B8 as u8;
                        cmd[DATA.start] = *byte;
                    }

                    Data::U16(halfword) => {
                        cmd[FORMAT] = Format::B16 as u8;
                        // XXX is this the right endianness?
                        cmd[DATA.start..DATA.start + 2].copy_from_slice(&halfword.to_le_bytes());
                    }

                    Data::U32(word) => {
                        cmd[FORMAT] = Format::B32 as u8;
                        // XXX is this the right endianness?
                        cmd[DATA].copy_from_slice(&word.to_le_bytes());
                    }
                }

                0x02
            }
        };

        cmd[CMD_TY1] = cmdty;
        cmd[CMD_TY2] = cmdty;

        bytes
    }
}
