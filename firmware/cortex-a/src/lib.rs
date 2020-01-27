//! Low level access to Cortex-A processors

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod register;

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
pub fn delay(n: u32) {
    extern "C" {
        fn __delay(n: u32);

    }

    unsafe { __delay(n) }
}

/// Enables FIQ interrupts
///
/// # Safety
///
/// This operation can break critical sections based on masking FIQ
pub unsafe fn enable_fiq() {
    extern "C" {
        fn __enable_fiq();
    }

    __enable_fiq()
}

/// Disable FIQ interrupts
pub fn disable_fiq() {
    extern "C" {
        fn __disable_fiq();
    }

    unsafe { __disable_fiq() }
}

/// Enables IRQ interrupts
///
/// # Safety
///
/// This operation can break critical sections based on masking IRQ
pub unsafe fn enable_irq() {
    extern "C" {
        fn __enable_irq();
    }

    __enable_irq()
}

/// Disable IRQ interrupts
pub fn disable_irq() {
    extern "C" {
        fn __disable_irq();
    }

    unsafe { __disable_irq() }
}

/// Data Memory Barrier
pub fn dmb() {
    extern "C" {
        fn __dmb();
    }

    unsafe { __dmb() }
}

/// Data Synchronization Barrier
pub fn dsb() {
    extern "C" {
        fn __dsb();
    }

    unsafe { __dsb() }
}

/// Instruction Synchronization Barrier
pub fn isb() {
    extern "C" {
        fn __isb();
    }

    unsafe { __isb() }
}

/// Wait For Interrupt
pub fn wfi() {
    extern "C" {
        fn __wfi();
    }

    unsafe { __wfi() }
}
