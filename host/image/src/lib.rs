//! i.MX6 program image reader / writer
//!
//! # References
//!
//! - section 8.7 of i.MX 6UltraLite Applications Processor Reference Manual

#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use core::fmt;

use nom::{number::complete as number, IResult};

pub use crate::{dcd::Dcd, ivt::Ivt};

pub mod dcd;
pub mod ivt;
pub mod read;
pub mod write;

/// Boot Data section
pub struct BootData {
    /// Start address of the boot data
    pub start: u32,

    /// Length of the boot data
    pub len: u32,

    /// Plugin flag
    pub plugin: u32,
}

impl BootData {
    const SIZE: u8 = 12;

    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, start) = number::le_u32(input)?;
        let (input, len) = number::le_u32(input)?;
        let (input, plugin) = number::le_u32(input)?;

        Ok((input, BootData { start, len, plugin }))
    }
}

impl fmt::Debug for BootData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BootData")
            .field("start", &Hex(self.start))
            .field("len", &self.len)
            .field("plugin", &self.plugin)
            .finish()
    }
}

struct Hex(u32);

impl fmt::Debug for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}
