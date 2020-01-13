//! **usbarmory.rs**: board support package for USB armory mkII devices from F-Secure

#![no_std]
#![doc(html_root_url = "https://docs.rs/usbarmory/0.0.0")]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub use cortex_a::delay;
use rac::wdog;
use usbarmory_rt as _;

pub mod led;
pub mod serial;

/// Default CPU frequency
///
/// Useful to generate `delay`s in seconds, e.g. `delay(5 * CPU_FREQUENCY)`
/// produces a delay of at least 5 seconds
pub const CPU_FREQUENCY: u32 = 528_000_000;

/// Uses the watchdog to reset the SoC
///
/// This is useful to return to the u-boot console during development
pub fn reset() -> ! {
    unsafe {
        let old = wdog::WDOG1_WCR.read_volatile();
        wdog::WDOG1_WCR.write_volatile(old & !wdog::WDOG_WCR_SRS);
    }

    // the watchdog reset may not be instantaneous so we use an infinite-loop
    // "trap" to give it some time and satisfy the signature of the diverging
    // function
    loop {
        continue;
    }
}
