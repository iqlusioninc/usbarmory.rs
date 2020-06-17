//! Test the FPU
//!
//! NOTE you must use the `armv7a-none-eabihf` target to use the hardware FPU;
//! the other target, `armv7a-none-eabihf`, will perform floating point
//! operations in software

#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{println, serial::Serial};

static X: AtomicU32 = AtomicU32::new(1065353216); // 1.0
static Y: AtomicU32 = AtomicU32::new(1065353216); // 1.0

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let z: f32 =
        f32::from_bits(X.load(Ordering::Relaxed)) + f32::from_bits(Y.load(Ordering::Relaxed));
    println!("{}", z);
    Serial::flush();

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
