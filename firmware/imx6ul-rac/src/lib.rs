//! (Temporary) Register Access Crate for the i.MX6UL SoC
//!
//! # References
//!
//! - (RM) i.MX 6UltraLite ApplicationsProcessor Reference Manual (IMX6ULRM)

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

// TODO this should be auto-generated

pub mod gic;
pub mod gpio;
pub mod src;
pub mod uart;
pub mod wdog;
