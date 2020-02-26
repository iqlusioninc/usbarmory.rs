//! **usbarmory.rs**: board support package for USB armory mkII devices from F-Secure
//!
//! # References
//!
//! - 'ULRM': i.MX 6UltraLite Applications Processor Reference Manual (IMX6ULRM)
//! - 'ULLRM': i.MX 6ULL Applications ProcessorReference Manual (IMX6ULLRM)

#![no_std]
#![doc(
    html_logo_url = "https://storage.googleapis.com/iqlusion-production-web/github/usbarmory/usbarmory-ferris.png",
    html_root_url = "https://docs.rs/usbarmory/0.0.0"
)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub use cortex_a::delay;
pub use memlog::memlog;
use pac::wdog::WDOG1;
use usbarmory_rt as _;

use crate::serial::Serial;

pub mod emmc;
pub mod led;
mod macros;
pub mod rng;
pub mod serial;
pub mod time;
pub mod usbd;
mod util;

/// Default CPU frequency
///
/// Useful to generate `delay`s in seconds, e.g. `delay(5 * CPU_FREQUENCY)`
/// produces a delay of at least 5 seconds
pub const CPU_FREQUENCY: u32 = 528_000_000;

/// Uses the watchdog to reset the SoC
///
/// This is useful to return to the u-boot console during development
pub fn reset() -> ! {
    /// Software Reset Signal
    pub const WDOG_WCR_SRS: u16 = 1 << 4;

    cortex_a::disable_irq();

    // NOTE(borrow_unchecked) interrupts have been disabled; we are now in a
    // critical section
    WDOG1::borrow_unchecked(|wdog| {
        let old = wdog.WCR.read();
        wdog.WCR.write(old & !WDOG_WCR_SRS);
    });

    // the watchdog reset may not be instantaneous so we use an infinite-loop
    // "trap" to give it some time and satisfy the signature of the diverging
    // function
    loop {
        continue;
    }
}

/// Implementation detail
pub fn memlog_flush_and_reset(file: &str, line: u32) -> ! {
    cortex_a::disable_irq();

    // called twice to handle the wrap-around case
    for _ in 0..4 {
        memlog::peek(true, |s| {
            // NOTE(borrow_unchecked) this runs with interrupts disabled (critical
            // section)
            Serial::borrow_unchecked(|serial| serial.write_all(s));
            s.len()
        });
    }

    Serial::borrow_unchecked(|mut serial| {
        use core::fmt::Write;
        write!(serial, "\n\rmemlog_flush_and_reset @ {}:{}\n\r", file, line).ok();
    });

    Serial::flush();

    reset()
}

/// [Non-blocking] Transmits some of the contents of the in-memory logger over
/// the serial interface
pub fn memlog_try_flush() {
    memlog::peek(false, |s| {
        Serial::borrow_unchecked(|serial| serial.try_write_all(s))
    })
}
