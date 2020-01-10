//! Serial interface

use core::fmt;

use rac::uart;

/// Handle to the serial interface
pub struct Serial {
    _private: (),
}

impl Serial {
    /// FIXME once we get interrupts this needs to return `Option<Self>`
    // NOTE u-boot already initialized this
    pub fn get() -> Self {
        Serial { _private: () }
    }

    /// Blocks until all data has been transmitted
    pub fn flush(&mut self) {
        unsafe { while uart::UART2_USR2.read_volatile() & uart::UART_USR2_TXDC == 0 {} }
    }

    /// [Blocking] Sends the given `bytes` through the serial interface
    pub fn write_all(&mut self, bytes: &[u8]) {
        for byte in bytes {
            unsafe {
                // if the FIFO buffer is full wait until we can write the next byte
                while uart::UART2_USR1.read_volatile() & uart::UART_USR1_TRDY == 0 {}

                uart::UART2_UTXD.write_volatile(*byte as u32);
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
