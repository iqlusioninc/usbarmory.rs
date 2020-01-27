//! Serial interface

use core::{
    fmt,
    marker::PhantomData,
    sync::atomic::{AtomicU8, Ordering},
};

use pac::uart::UART2;

const NEVER: u8 = 0; // never taken
const TAKEN: u8 = 1; // currently taken
const FREE: u8 = 2; // free to take
static STATE: AtomicU8 = AtomicU8::new(NEVER);

/// Events that can trigger an interrupt
pub enum Event {
    /// RxFIFO contains data
    ReceiveReady,
}

/// Handle to the serial interface
pub struct Serial {
    _not_sync: PhantomData<*mut ()>,
}

unsafe impl Send for Serial {}

impl Serial {
    unsafe fn new() -> Self {
        Serial {
            _not_sync: PhantomData,
        }
    }

    /// Gets an exclusive handle to the `Serial` singleton
    pub fn take() -> Option<Self> {
        if STATE.load(Ordering::Acquire) == NEVER
            && STATE
                .compare_exchange(NEVER, TAKEN, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        {
            return UART2::take().map(|uart| {
                // u-boot already initialized this
                drop(uart); // this seals the UART2 configuration

                unsafe { Serial::new() }
            });
        }

        if STATE
            .compare_exchange(FREE, TAKEN, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            Some(unsafe { Serial::new() })
        } else {
            None
        }
    }

    /// Gets a handle to the `Serial` singleton even if it's currently owned by
    /// some other context
    ///
    /// WARNING: using `borrow_unchecked` + `write_all` in concurrent contexts
    /// can result in data loss
    // NOTE(safety) at the moment this is not unsound because `Serial` only uses
    // MMIO registers which have strongly-ordered, single-instruction load and
    // store (but not Read-Modify-Write) operations. If the abstraction at some
    // point starts *internally* using normal memory (e.g. a buffer in RAM) then
    // this operation would need to become `unsafe`.
    pub fn borrow_unchecked<R>(f: impl FnOnce(&Self) -> R) -> R {
        f(unsafe { &Serial::new() })
    }

    /// Release the exclusive handle so any other context can take it
    pub fn release(self) {
        STATE.store(FREE, Ordering::Release);
    }

    /// Blocks until all data has been transmitted
    pub fn flush() {
        /// Transmitter Complete
        const UART_USR2_TXDC: u32 = 1 << 3;

        // NOTE(borrow_unchecked) reading `USR2` has no side effects
        UART2::borrow_unchecked(|uart| {
            while uart.USR2.read() & UART_USR2_TXDC == 0 {
                // busy wait
                continue;
            }
        })
    }

    /// Starts listening for a event
    ///
    /// `event` will now trigger interrupts
    pub fn listen(&self, event: Event) {
        // NOTE(borrow_unchecked) the `UART2` singleton has been dropped; only
        // the owner of `Serial` can access the peripheral
        UART2::borrow_unchecked(|uart| {
            let old = uart.UCR2.read();
            match event {
                Event::ReceiveReady => {
                    uart.UCR1.write(old | (1 << 9));
                }
            }
        });
    }

    /// Reads a single byte from the serial interface
    ///
    /// Returns `None` if no data is currently available
    pub fn read(&self) -> Option<u8> {
        /// Receiver Ready Interrupt
        const UART_USR1_RRDY: u32 = 1 << 9;

        // NOTE(borrow_unchecked) the `UART2` singleton has been dropped; only
        // the owner of `Serial` can access the peripheral
        UART2::borrow_unchecked(|uart| {
            if uart.USR1.read() & UART_USR1_RRDY == 0 {
                None
            } else {
                Some(uart.URXD.read() as u8)
            }
        })
    }

    /// [Blocking] Sends a single `byte` through the serial interface
    pub fn write(&self, byte: u8) {
        /// Transmitter Ready Interrupt
        const UART_USR1_TRDY: u32 = 1 << 13;

        // NOTE(borrow_unchecked) the `UART2` singleton has been dropped; only
        // the owner of `Serial` can access the peripheral
        UART2::borrow_unchecked(|uart| {
            // if the FIFO buffer is full wait until we can write the next byte
            while uart.USR1.read() & UART_USR1_TRDY == 0 {
                // busy wait
                continue;
            }

            uart.UTXD.write(byte as u32);
        })
    }

    /// [Blocking] Sends the given `bytes` through the serial interface
    pub fn write_all(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write(*byte);
        }
    }
}

impl fmt::Write for &'_ Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}
