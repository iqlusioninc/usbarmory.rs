//! **usbarmory.rs**: board support package for USB armory mkII devices from F-Secure

#![no_std]
#![doc(html_root_url = "https://docs.rs/usbarmory/0.0.0")]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use core::fmt::Write as _;

use rac::src;
use usbarmory_rt as _;
pub use cortex_a::delay;

use crate::serial::Serial;

pub mod led;
pub mod serial;

/// Software resets the Cortex-A core
// FIXME this doesn't get us back to the u-boot console; i.e. it doesn't cause
// the boot ROM to run again
pub fn reset() -> ! {
    unsafe {
        let old = src::SRC_SCR.read_volatile();
        src::SRC_SCR.write_volatile(old | src::SRC_SCR_CORE0_RST_MASK);
    }

    // TODO replace with `unreachable_unchecked`
    writeln!(Serial::get(), "reset failed").ok();
    loop {
        continue;
    }
}
