//! Cargo runner for loading and running Rust programs in a u-boot-less environment

use core::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use std::{
    env, fs,
    io::{self, Write},
    thread,
};

use anyhow::{bail, format_err, Context};
use image::write::Image;
use serialport::{SerialPortSettings, SerialPortType};
use xmas_elf::ElfFile;

use usd::Usd;

const COLD_BOOT_ERROR: &str = "Potential COLD_BOOT error

Likely fix: power cycle the board and retry with the `COLD_BOOT` env var set to `1` but
only set it for the *first* Cargo runner invocation. That is:

$ # power cycle the board then run
$ COLD_BOOT=1 cargo run --example foo

$ # omit the env var (or unset it) for the rest of invocations
$ cargo run --example bar";

fn main() -> Result<(), anyhow::Error> {
    // NOTE(skip) program name
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    let bytes = fs::read(&args[0])?;
    let elf = ElfFile::new(&bytes).map_err(|s| format_err!("{}", s))?;

    // wait until the USB device is available
    let mut usd = Usd::open()?;
    usd.set_verbose(true);
    let usb_address = usd.usb_address();

    thread::spawn(|| {
        if let Err(e) = redirect() {
            eprintln!("serial interface error: {}", e);
        }
    });

    // do not include a DCD in the image because we'll send it over USB
    let skip_dcd = true;
    let image = Image::from_elf(&elf, skip_dcd)?;

    // cold boot: include the DCD to initialize DDR RAM
    // warm reset: omit the DCD because DDR RAM is already initialized (and trying to re-initialize
    // will hang the Armory)
    let cold_boot = env::var_os("COLD_BOOT").is_some();

    if cold_boot {
        // DCD to initialize the external DDR RAM
        let dcd = image::write::init_ddr();
        usd.dcd_write(usd::OCRAM_FREE_ADDRESS, &dcd.into_bytes())
            .context(COLD_BOOT_ERROR)?;
    }

    let address = image.self_address();
    let res = usd.write_file(address, &image.into_bytes());
    if !cold_boot {
        res.context(COLD_BOOT_ERROR)?
    } else {
        res?
    }
    usd.jump_address(address)?;

    // the program is running now; if we see the USB device get re-enumerated it means the Armory
    // rebooted back into USD mode
    loop {
        if usd::util::has_been_reenumerated(usb_address) {
            // stop the `redirect` thread and terminate this process
            // but give it some time to flush any remaining data
            thread::sleep(Duration::from_millis(100));
            STOP.store(true, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(100));
            eprintln!("(device has reset)");
            return Ok(());
        } else {
            thread::sleep(Duration::from_millis(100))
        }
    }
}

static STOP: AtomicBool = AtomicBool::new(false);

/// Redirects serial data to the console
fn redirect() -> Result<(), anyhow::Error> {
    const VID: u16 = 0x0403; // Future Technology Devices International, Ltd
    const PID: u16 = 0x6011; // FT4232H Quad HS USB-UART/FIFO IC
    const DEVNO: usize = 2; // device #3 is the one we want
    const BAUD_RATE: u32 = 4_000_000;
    const BUFSZ: usize = 512; // the FT4232H uses 512B USB packets

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = BAUD_RATE;

    let mut candidates = vec![];
    for info in serialport::available_ports()?.into_iter() {
        if let SerialPortType::UsbPort(usb) = info.port_type {
            if usb.vid == VID && usb.pid == PID {
                candidates.push(info.port_name);
            }
        }
    }

    if let Some(path) = candidates.get(DEVNO) {
        let mut serial = serialport::open_with_settings(path, &settings)?;

        let mut stdout = io::stdout();
        let mut buf = [0; BUFSZ];

        while !STOP.load(Ordering::Relaxed) {
            if serial.bytes_to_read()? != 0 {
                let len = serial.read(&mut buf)?;
                stdout.write_all(&buf[..len])?;
            } else {
                // the span of one USB micro-frame
                thread::sleep(Duration::from_micros(125));
            }
        }
    } else {
        bail!(
            "USB device {:04x}:{:04x} (serial interface) was not found",
            VID,
            PID
        )
    }

    Ok(())
}
