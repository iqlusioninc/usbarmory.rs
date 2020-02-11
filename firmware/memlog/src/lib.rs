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
//! use the `nb_memlog_flush` and `memlog_flush_to_end` API provided in the
//! `usbarmory` crate.

#![no_std]

use core::{
    cell::UnsafeCell,
    cmp, fmt, ptr, slice,
    sync::atomic::{AtomicUsize, Ordering},
};

use pac::gicc::GICC;

/// Hard-coded buffer capacity
const N: usize = 8 * 1024;

static mut BUFFER: UnsafeCell<[u8; N]> = UnsafeCell::new([0; N]);
static READ: AtomicUsize = AtomicUsize::new(0);
static WRITE: AtomicUsize = AtomicUsize::new(0);

/// Implementation detail
#[doc(hidden)]
pub struct Logger;

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        log(s);
        Ok(())
    }
}

/// Implementation details
#[doc(hidden)]
pub fn log(s: &str) {
    if !in_main() {
        return;
    }

    let write = WRITE.load(Ordering::Acquire);

    let bufferp = unsafe { BUFFER.get() as *mut u8 };
    let read = READ.load(Ordering::Relaxed);
    let bytes = s.as_bytes();
    let n = bytes.len();
    unsafe {
        assert!(n <= read.wrapping_add(N).wrapping_sub(write));

        let cursor = write % N;
        if cursor + n > N {
            // wrap-around: do a split memcpy
            let pivot = cursor + n - N;

            // until the end of `BUFFER`
            ptr::copy_nonoverlapping(bytes.as_ptr(), bufferp.add(cursor), pivot);

            // from the start of `BUFFER`
            ptr::copy_nonoverlapping(bytes.as_ptr().add(pivot), bufferp, n - pivot);
        } else {
            // single memcpy
            ptr::copy_nonoverlapping(bytes.as_ptr(), bufferp.add(cursor), n);
        }

        WRITE.store(write + n, Ordering::Release);
    }
}

/// Peeks into the memory buffer
///
/// The buffer will be advanced by the amount of read bytes reported by the
/// closure `f`
///
/// This will panic if it's called in interrupt context
pub fn peek(f: impl FnOnce(&[u8]) -> usize) {
    assert!(in_main());

    let read = READ.load(Ordering::Acquire);

    let bufferp = unsafe { &BUFFER as *const _ as *const u8 };
    let write = WRITE.load(Ordering::Relaxed);
    // NOTE(cmp::min) avoid exceeding the boundary of the buffer
    let n = cmp::min(write.wrapping_sub(read), N - (read % N));
    let cursor = read % N;
    let bytes_read = f(unsafe { slice::from_raw_parts(bufferp.add(cursor), n) });

    // NOTE(cmp::min) guard against the closure reported more bytes read than
    // the amount that it was shown
    READ.store(read + cmp::min(bytes_read, n), Ordering::Release);
}

// Or "not in interrupt context"
fn in_main() -> bool {
    // "main" runs at the lowest priority of `0xff` (hardware priority)
    GICC::borrow_unchecked(|gicc| gicc.RPR.read()) == 0xff
}

/// Logs the formatted string into the device memory
///
/// NOTE this is a no-op in interrupt context
#[macro_export]
macro_rules! memlog {
    ($s:expr) => {
        $crate::log(concat!($s, "\n\r"));
    };

    ($s:expr, $($args:tt)*) => {{
        use core::fmt::Write as _;
        let _ = write!($crate::Logger, concat!($s, "\n\r"), $($args)*); // never errors
    }};
}
