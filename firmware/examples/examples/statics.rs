//! Sanity check that static variables work
//!
//! Expected output:
//!
//! ```
//! X=0, Y=1
//! X=1, Y=2
//! ```

#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

// .bss
static X: AtomicU64 = AtomicU64::new(0);
// .data
static Y: AtomicU64 = AtomicU64::new(1);

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    println!(
        "X={}, Y={}",
        X.load(Ordering::Relaxed),
        Y.load(Ordering::Relaxed)
    );

    X.fetch_add(1, Ordering::Relaxed);
    Y.fetch_add(1, Ordering::Relaxed);

    println!(
        "X={}, Y={}",
        X.load(Ordering::Relaxed),
        Y.load(Ordering::Relaxed)
    );

    Serial::flush();

    usbarmory::reset()
}
