//! Basic usage pattern of the in-memory logger
//!
//! Expected output:
//!
//! ```
//! Hello, world!
//! The answer is 42
//!
//! memlog_flush_and_reset @ usbarmory/examples/memlog.rs:38
//! ```

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{memlog, memlog_flush_and_reset};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    // these operations complete fast
    memlog!("Hello, world!");
    memlog!("The answer is {}", 42);

    while wait_on_some_io() {
        // try to flush some data while waiting for something else.
        // If this is never done the in-memory logger could become full and that
        // triggers a `panic!`
        usbarmory::memlog_try_flush();
    }

    // this transmit all the data that remains in the in-memory logger over the
    // serial interface. Once the transmission is done the system is reset
    memlog_flush_and_reset!();
}

fn wait_on_some_io() -> bool {
    false
}
