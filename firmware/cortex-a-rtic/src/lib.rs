//! Real Time for The Masses

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub use cortex_a_rtic_macros::app;
pub use rtic_core::Mutex;
use usbarmory_rt as _; // memory layout & exception vector

/// Implementation details
#[doc(hidden)]
pub mod export;
