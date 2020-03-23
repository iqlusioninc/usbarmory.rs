//! Cargo runner for loading and running Rust programs in a u-boot-less environment

use core::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use std::{
    env, fs,
    io::{self, Write},
    process::{Command, Stdio},
    thread,
};

use anyhow::{bail, format_err};
use image::write::Image;
use serialport::SerialPortSettings;
use tempfile::NamedTempFile;
use xmas_elf::ElfFile;

use crate::sdp::Sdp;

mod hid;
#[allow(dead_code)]
mod sdp;

fn main() -> Result<(), anyhow::Error> {
    // NOTE(skip) program name
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    let bytes = fs::read(&args[0])?;
    let elf = ElfFile::new(&bytes).map_err(|s| format_err!("{}", s))?;

    // wait until the USB device is available
    let sdp = Sdp::open()?;
    let usb_address = sdp.usb_address();

    thread::spawn(|| {
        if let Err(e) = redirect() {
            eprintln!("serial interface error: {}", e);
        }
    });

    // cold boot: include the DCD to initialize DDR RAM
    // warm reset: omit the DCD because DDR RAM is already initialized (and trying to re-initialize
    // will hang the Armory)
    let skip_dcd = env::var_os("COLD_BOOT").is_none();

    // write the program image to disk because `imx_usb` needs a path to it
    let image = Image::from_elf(&elf, skip_dcd)?;
    let mut file = NamedTempFile::new()?;
    image.write(&mut file)?;

    // release the USB device before calling `imx_usb`
    drop(sdp);

    let status = Command::new("imx_usb")
        .arg(file.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        bail!(
            "`imx_usb` failed
Possible fix: power cycle the board and retry with the `COLD_BOOT` env var set to `1` but
only set it for the *first* Cargo runner invocation. That is:

$ # power cycle the board then run
$ COLD_BOOT=1 cargo run --example foo

$ # omit the env var (or unset it) for the rest of invocations
$ cargo run --example bar"
        );
    }

    // the program is running now; if we see the SDP device get re-enumerated it means the Armory
    // rebooted back into SDP mode
    loop {
        if Sdp::reconnected(usb_address) {
            // stop the `redirect` thread and terminate this process
            // but give it some time to flush any remaining data
            thread::sleep(Duration::from_millis(100));
            STOP.store(true, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(100));
            eprintln!("device has reset");
            return Ok(());
        } else {
            thread::sleep(Duration::from_millis(100))
        }
    }
}

static STOP: AtomicBool = AtomicBool::new(false);

/// Redirects serial data to the console
fn redirect() -> Result<(), anyhow::Error> {
    // FIXME this should look for the right port using `serialport::available_ports`
    #[cfg(target_os = "linux")]
    const PATH: &str = "/dev/ttyUSB2";
    #[cfg(not(target_os = "linux"))]
    compile_error!(
        "non-Linux host: path to serial device (debug accessory) must be entered into the program"
    );

    const BAUD_RATE: u32 = 4_000_000;
    // the FT4232H uses 512B USB packets
    const BUFSZ: usize = 512;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = BAUD_RATE;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = [0; BUFSZ];
    if let Ok(mut serial) = serialport::open_with_settings(PATH, &settings) {
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
        eprintln!("warning: serial interface couldn't be opened");
    }

    Ok(())
}
