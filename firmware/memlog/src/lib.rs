//! In-memory logger
//!
//! Logging over serial can block for prolonged periods of time because the
//! processor has to wait until some of the data has been put on the wire before
//! it can write new data into the serial port FIFO buffer.
//!
//! In time-critical context it's better to just copy the logs into memory and
//! defer the act of transmitting them over serial to a non time-critical
//! context. This is what this in-memory logger lets you do.
//!
//! You should only use the `memlog!` macro defined here. To transmit the data
//! use the `try_memlog_flush` and `memlog_flush_and_reset` API provided in the
//! `usbarmory` crate.
//!
//! Note that `memlog!` works in thread mode (AKA `main`) and interrupt context
//! but should *not* be used from interrupts that run at different priority.
//! This will result in data loss.

#![no_std]

use core::{
    cmp, fmt, ptr, slice,
    sync::atomic::{AtomicUsize, Ordering},
};

use pac::GICC;

// End users must only ever modify these two `consts`
const N0: usize = 8 * 1024; // size of circular buffer @ priority 0
const N1: usize = 8 * 1024; // size of circular buffer @ priority !0

#[link_section = ".uninit.memlog_B0"]
static mut B0: [u8; N0] = [0; N0];

#[link_section = ".uninit.memlog_B0"]
static mut B1: [u8; N1] = [0; N1];

struct Buffer {
    bufferp: *mut u8,
    cap: usize,
    read: AtomicUsize,
    write: AtomicUsize,
}

impl Buffer {
    /// # Safety
    /// Caller must ensure tha `bufferp` points into an array that's at least `cap` bytes big
    /// Caller must manually enforce Rust aliasing rules
    const unsafe fn new(cap: usize, bufferp: *mut u8) -> Self {
        Self {
            bufferp,
            cap,
            read: AtomicUsize::new(0),
            write: AtomicUsize::new(0),
        }
    }
}

impl Buffer {
    fn push(&self, bytes: &[u8]) {
        let bufferp = self.bufferp;
        let write = self.write.load(Ordering::Acquire);
        let cap = self.cap;

        let read = self.read.load(Ordering::Relaxed);
        let n = bytes.len();
        assert!(
            n <= read.wrapping_add(cap).wrapping_sub(write),
            "memlog is full; maybe try a bigger buffer? see memlog/src/lib.rs"
        );

        let cursor = write % cap;
        unsafe {
            if cursor + n > cap {
                // wrap-around: do a split memcpy
                let pivot = cursor + n - cap;

                // until the end of `BUFFER`
                ptr::copy_nonoverlapping(bytes.as_ptr(), bufferp.add(cursor), pivot);

                // from the start of `BUFFER`
                ptr::copy_nonoverlapping(bytes.as_ptr().add(pivot), bufferp, n - pivot);
            } else {
                // single memcpy
                ptr::copy_nonoverlapping(bytes.as_ptr(), bufferp.add(cursor), n);
            }
        }

        self.write.store(write + n, Ordering::Release);
    }

    fn peek(&self, f: &mut Option<impl FnOnce(&[u8]) -> usize>) {
        let bufferp = self.bufferp;
        let read = self.read.load(Ordering::Acquire);
        let write = self.write.load(Ordering::Relaxed);
        let cap = self.cap;

        // NOTE(cmp::min) avoid exceeding the boundary of the buffer
        let n = cmp::min(write.wrapping_sub(read), cap - (read % cap));
        let cursor = read % cap;
        if n != 0 {
            let f = if let Some(f) = f.take() { f } else { return };
            let bytes_read = unsafe { f(slice::from_raw_parts(bufferp.add(cursor), n)) };

            // NOTE(cmp::min) guard against the closure reported more bytes read than
            // the amount that it was shown
            self.read
                .store(read + cmp::min(bytes_read, n), Ordering::Release);
        }
    }
}

/// Implementation detail
#[doc(hidden)]
pub struct Logger;

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        log(s);
        Ok(())
    }
}

static mut L0: Buffer = unsafe { Buffer::new(N0, &mut B0 as *mut _ as *mut u8) };
static mut L1: Buffer = unsafe { Buffer::new(N1, &mut B1 as *mut _ as *mut u8) };

/// Implementation details
#[doc(hidden)]
pub fn log(s: &str) {
    let bytes = s.as_bytes();
    unsafe {
        if in_main() {
            L0.push(bytes);
        } else {
            L1.push(bytes);
        }
    }
}

/// Peeks into the memory buffer
///
/// The buffer will be advanced by the amount of read bytes reported by the
/// closure `f`
///
/// This will do nothing if called from interrupt context
pub fn peek(all: bool, f: impl FnOnce(&[u8]) -> usize) {
    unsafe {
        if all || in_main() {
            let mut f = Some(f);
            L1.peek(&mut f);
            L0.peek(&mut f);
        }
    }
}

// Or "not in interrupt context"
fn in_main() -> bool {
    // "main" runs at the lowest priority of `0xff` (hardware priority)
    GICC::borrow_unchecked(|gicc| gicc.RPR.read()) == 0xff
}

/// Logs the formatted string into the device memory
#[macro_export]
macro_rules! memlog {
    ($s:expr) => {
        $crate::log(concat!($s, "\n"));
    };

    ($s:expr, $($args:tt)*) => {{
        use core::fmt::Write as _;
        let _ = write!($crate::Logger, concat!($s, "\n"), $($args)*); // never errors
    }};
}
