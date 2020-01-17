//! A program that produces a stack overflow
//!
//! There's stack overflow protection installed so this won't cause undefined
//! behavior. Stack overflows will trigger a "data abort" (hardware) exception.
//!
//! Expected output (output may not match exactly due to optimization
//! differences between compiler versions):
//!
//! ```
//! fib(808, 0x900610)
//! fib(806, 0x9001b8)
//! fib(
//! data abort exception (it could indicate a stack overflow)
//! ```
//!
//! Then the device will reboot

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::println;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let x = fib(1024);
    println!("{}", x);

    usbarmory::reset();
}

// Recursive function
#[inline(never)]
fn fib(n: u32) -> u32 {
    // force a 1KB stack allocation on each call
    let mut x = [0u8; 1024];
    println!("fib({}, {:?})", n, x.as_mut_ptr());

    if n < 2 {
        1
    } else {
        fib(n - 2).wrapping_add(fib(n - 1))
    }
}
