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
    cell::UnsafeCell,
    cmp, fmt,
    mem::MaybeUninit,
    ptr, slice,
    sync::atomic::{AtomicUsize, Ordering},
};

use generic_array::{typenum::consts, ArrayLength, GenericArray};
use pac::gicc::GICC;

// End users must only ever modify the `consts` type parameter of `B0` and `B1`

/// Circular buffer @ priority 0
static mut B0: Buffer<BigArray<consts::U2>> = Buffer::new();
//                                     ^^

/// Circular buffer @ priority !0
static mut B1: Buffer<BigArray<consts::U8>> = Buffer::new();
//                                     ^^

/// Hard-coded buffer capacity
const M: usize = 1024;

struct Buffer<A> {
    read: AtomicUsize,
    write: AtomicUsize,
    buffer: UnsafeCell<MaybeUninit<A>>,
}

impl<A> Buffer<A> {
    const fn new() -> Self {
        Self {
            read: AtomicUsize::new(0),
            write: AtomicUsize::new(0),
            buffer: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

type BigArray<N> = GenericArray<[u8; 1024], N>;

impl<N> Buffer<BigArray<N>>
where
    N: ArrayLength<[u8; 1024]>,
{
    fn push(&self, bytes: &[u8]) {
        let bufferp = self.buffer.get() as *mut u8;
        let write = self.write.load(Ordering::Acquire);
        let cap = M * N::USIZE;

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
        let bufferp = self.buffer.get() as *const u8;
        let read = self.read.load(Ordering::Acquire);
        let write = self.write.load(Ordering::Relaxed);
        let cap = M * N::USIZE;

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

/// Implementation details
#[doc(hidden)]
pub fn log(s: &str) {
    let bytes = s.as_bytes();
    unsafe {
        if in_main() {
            B0.push(bytes);
        } else {
            B1.push(bytes);
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
            B1.peek(&mut f);
            B0.peek(&mut f);
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
        $crate::log(concat!($s, "\n\r"));
    };

    ($s:expr, $($args:tt)*) => {{
        use core::fmt::Write as _;
        let _ = write!($crate::Logger, concat!($s, "\n\r"), $($args)*); // never errors
    }};
}
