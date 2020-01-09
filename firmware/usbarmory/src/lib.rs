//! **usbarmory.rs**: board support package for USB armory mkII devices from F-Secure

#![no_std]
#![doc(html_root_url = "https://docs.rs/usbarmory/0.0.0")]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use core::fmt::{self, Write as _};

use usbarmory_rt as _;

// Chapter 53 UART
const UART2_BASE: usize = 0x021e_8000;
// UART Transmitter Register
const UART2_UTXD: *mut u32 = (UART2_BASE + 0x40) as *mut _;
// UART Status Register
const UART2_USR1: *mut u32 = (UART2_BASE + 0x94) as *mut _;
// Transmitter Ready Interrupt
const UART_USR1_TRDY: u32 = 1 << 13;

/// Handle to the serial interface
pub struct Serial {
    _private: (),
}

impl Serial {
    /// FIXME once we get interrupts this needs to return `Option<Self>`
    pub fn get() -> Self {
        Serial { _private: () }
    }

    /// [Blocking] Sends the given `bytes` through the serial interface
    pub fn write_all(&mut self, bytes: &[u8]) {
        for byte in bytes {
            unsafe {
                // if the FIFO buffer is full wait until we can write the next byte
                while UART2_USR1.read_volatile() & UART_USR1_TRDY == 0 {}

                UART2_UTXD.write_volatile(*byte as u32);
            }
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}

/// Software resets the Cortex-A core
// FIXME this doesn't get us back to the u-boot console; i.e. it doesn't cause
// the boot ROM to run again
pub fn reset() -> ! {
    /// System Reset Controller
    const SRC_BASE: usize = 0x020d_8000;
    /// SRC Control Register
    const SRC_SCR: *mut u32 = (SRC_BASE + 0x0) as *mut _;
    // const SRC_SCR_CORES_DBG_RST_MASK: u32 = 1 << 21;
    const SRC_SCR_CORE0_RST_MASK: u32 = 1 << 13;

    unsafe {
        let old = SRC_SCR.read_volatile();
        SRC_SCR.write_volatile(old | SRC_SCR_CORE0_RST_MASK);
    }

    // TODO replace with `unreachable_unchecked`
    writeln!(Serial::get(), "reset failed").ok();
    loop {}
}
