//! ELF upload utility for U-Boot.
//!
//! This tool will talk to U-Boot over a serial port (`ttyUSB2` by default) and commands it to
//! accept a binary upload. It can transfer an ELF file to memory and then boot from it.
//!
//! Remember to close any terminal when using this, otherwise things break in weird ways.

use serialport::{SerialPort, SerialPortSettings};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::File,
    io::{self, Read, Write},
    path::Path,
    thread,
    time::Duration,
};
use xmodem::Xmodem;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let path = args
        .next()
        .ok_or_else(|| format!("missing file name argument"))?;

    if let Some(extra) = args.next() {
        return Err(format!("extra argument: {}", extra.to_string_lossy()).into());
    }

    let file = File::open(&path)
        .map_err(|e| format!("failed to open '{}': {}", path.to_string_lossy(), e))?;

    let serial = env::var_os("SERIAL").unwrap_or(OsString::from("/dev/ttyUSB2"));

    let mut port = open_serial(&serial)
        .map_err(|e| format!("failed to open {}: {}", serial.to_string_lossy(), e))?;

    // Take control of U-Boot, putting it in a known state.
    let uboot_version = control_uboot(&mut *port)?;

    println!("Connected to U-Boot. Version: {}", uboot_version);
    println!("Enabling XMODEM transfer mode.");

    writeln!(port, "loadx")?;

    // Now U-Boot echos the command and prints a status message before actually switching to XMODEM
    // mode. Read 2 lines so the transfer doesn't get messed up.
    let mut lines = 2;
    while lines > 0 {
        let mut buf = [0; 1];
        port.read(&mut buf)?;
        print!("{}", buf[0] as char);
        if buf[0] == b'\n' {
            lines -= 1;
        }
    }

    println!("Transferring {}", path.to_string_lossy());

    // Increase timeout as XMODEM doesn't talk too often.
    port.set_timeout(Duration::from_secs(3))?;

    let mut reader = ReadProgress {
        read: 0,
        size: file.metadata()?.len(),
        inner: file,
    };

    // FIXME: Error should implement `Display`
    Xmodem::new()
        .send(&mut port, &mut reader)
        .map_err(|e| format!("transfer error: {:?}", e))?;
    println!();

    port.set_timeout(Duration::from_secs(1))?;

    writeln!(port, "bootelf")?;

    // Now we turn into a serial monitor, which is handy since the examples use that for logging.
    // We try to detect reboots by looking for a line with the U-Boot version (which is the first
    // thing U-Boot prints when it starts), and exit when that happens.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut port = DetectReboot {
        buf: Vec::new(),
        version: uboot_version,
        inner: port,
    };
    io::copy(&mut port, &mut stdout).ok();

    Ok(())
}

fn open_serial(serial: &OsStr) -> Result<Box<dyn SerialPort>> {
    let path = Path::new(serial);
    let mut logged_not_found = false;
    loop {
        let result = serialport::open_with_settings(
            &serial,
            &SerialPortSettings {
                baud_rate: 115200,
                timeout: Duration::from_millis(100),
                ..Default::default()
            },
        );

        // We'd like to handle non-yet-connected serial ports specially. Unfortunately the
        // serialport crate will give us an "Unknown" error instead of an `io::ErrorKind::NotFound`,
        // so we have to do it differently.
        match result {
            Ok(port) => return Ok(port),
            Err(e) => {
                if path.exists() {
                    // Probably some "real" error.
                    return Err(e.into());
                } else {
                    // Wait until device is plugged in.
                    if !logged_not_found {
                        logged_not_found = true;
                        println!("(waiting for {} to show up)", serial.to_string_lossy());
                    }

                    thread::sleep(Duration::from_millis(200));
                }
            }
        }
    }
}

fn control_uboot<P: Read + Write + ?Sized>(port: &mut P) -> Result<String> {
    const TRIES: u8 = 10;
    for i in 1..=TRIES {
        match try_control_uboot(port) {
            Ok(uboot_version) => return Ok(uboot_version),
            Err(e) => eprintln!("(attempt {}/{}: failed to control U-Boot: {})", i, TRIES, e),
        }
    }

    Err(format!("could not get U-Boot prompt after {} tries", TRIES).into())
}

/// Attempts to get to the U-Boot command line.
///
/// This may fail spuriously, for example when U-Boot isn't running yet, or when it has left-over
/// data in its input buffer. The caller should retry this multiple times.
///
/// Note that this will not work if secure boot is fully enabled, since that disables the command
/// line.
fn try_control_uboot<P: Read + Write + ?Sized>(port: &mut P) -> Result<String> {
    // Empty input buffer
    let mut buf = [0; 512];
    port.read(&mut buf).ok();

    let buf: &mut [_] = &mut [0; 512];
    let mut read: &mut [_];
    loop {
        port.write_all(b"version\n")?;
        read = read_until_timeout(port, buf)?;
        if !read.is_empty() {
            break;
        }
    }

    let s = String::from_utf8_lossy(read);

    let mut version = None;
    for line in s.lines() {
        let line = line.trim();
        if line.starts_with("U-Boot") {
            // Found the U-Boot version line
            version = Some(line);
        }
    }

    // The last line must be the U-Boot prompt "=>", otherwise something went wrong and we won't be
    // able to control it.
    let last = s.lines().last().unwrap().trim();
    if last != "=>" {
        return Err(format!("did not get to U-Boot prompt").into());
    }

    if let Some(version) = version {
        Ok(version.to_string())
    } else {
        Err(format!("output did not contain U-Boot version").into())
    }
}

fn read_until_timeout<'a, R: Read + ?Sized>(
    read: &mut R,
    buffer: &'a mut [u8],
) -> Result<&'a mut [u8]> {
    let mut count = 0;
    loop {
        match read.read(&mut buffer[count..]) {
            Ok(c) => count += c,
            Err(e) if e.kind() == io::ErrorKind::TimedOut => return Ok(&mut buffer[..count]),
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}

/// Prints transfer progress by wrapping a `Read` implementor.
struct ReadProgress<R> {
    inner: R,
    read: u64,
    size: u64,
}

impl<R: Read> Read for ReadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.inner.read(buf) {
            Ok(bytes) => {
                self.read += bytes as u64;
                print!("\r{:10} / {:10} B", self.read, self.size);
                Ok(bytes)
            }
            Err(e) => Err(e),
        }
    }
}

/// Detects system reboots by looking at U-Boot messages. Disconnects when a reboot is detected.
struct DetectReboot<R> {
    inner: R,
    buf: Vec<u8>,
    version: String,
}

impl<R: Read> Read for DetectReboot<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes = self.inner.read(buf)?;
        self.buf.extend_from_slice(&buf[..bytes]);

        if self.buf.contains(&b'\n') {
            for line in self.buf.split(|b| b == &b'\n' || b == &b'\r') {
                if line == self.version.as_bytes() {
                    eprintln!("\nReboot detected, disconnecting.");
                    return Ok(0); // EOF
                }
            }

            let last = self.buf.split(|b| b == &b'\n').last().unwrap();
            self.buf = last.to_vec();
        }

        Ok(bytes)
    }
}
