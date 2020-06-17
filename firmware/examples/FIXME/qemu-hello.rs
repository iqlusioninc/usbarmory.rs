// FIXME the initialization (pre-main) code in the usbarmory{,-rt} crates accesses hardware
// peripherals that aren't implemented in QEMU. This causes the initialization code to hang before
// `main` is reached

#![no_main]
#![no_std]

use core::{fmt::Write as _, panic::PanicInfo};

use cortex_m_semihosting::{debug, hio};
use usbarmory as _; // memory layout

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Ok(mut stdout) = hio::hstdout() {
        writeln!(stdout, "\n{}", info).ok();
    }

    debug::exit(debug::EXIT_FAILURE);

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
fn DefaultHandler() -> ! {
    debug::exit(debug::EXIT_FAILURE);

    loop {}
}
