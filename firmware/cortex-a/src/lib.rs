//! Low level access to Cortex-A processors

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

/// "No OPeration" instruction
///
/// Use this in "for loop" delays or the compiler will optimize away your delay
pub fn nop() {
    extern "C" {
        fn __nop();
    }

    unsafe {
        __nop();
    }
}
