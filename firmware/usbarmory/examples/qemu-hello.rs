#![no_main]
#![no_std]

// use core::fmt::Write as _;

use cortex_m_semihosting::{debug, hio};
use panic_halt as _;
use usbarmory as _; // memory layout

#[no_mangle]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    // less bloated (no core::fmt) version of the `writeln!` invocation below
    stdout.write_all(b"Hello, world!\n").unwrap();

    // writeln!(stdout, "Hello, world!").ok();

    // exit QEMU
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
