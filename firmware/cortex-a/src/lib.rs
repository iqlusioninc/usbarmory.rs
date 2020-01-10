//! Low level access to Cortex-A processors

#![feature(asm)]
#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

/// "No OPeration" instruction
///
/// Use this in "for loop" delays or the compiler will optimize away your delay
pub fn nop() {
    extern "C" {
        fn __nop();
    }

    unsafe { __nop() }
}

/// Performs `n` CPU instructions to add a delay to the program
///
/// Note that this function may result in a delay of *more* than `n` CPU clock
/// cycles due to time spent in higher priority interrupt handlers
#[inline(never)]
pub fn delay(n: u32) {
    extern "C" {
        fn __delay(n: u32);

    }

    unsafe { __delay(n) }
}
