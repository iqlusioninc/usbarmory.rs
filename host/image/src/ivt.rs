//! Image vector table

use core::fmt;

use nom::{branch, bytes::complete as bytes, number::complete as number, IResult};

use crate::Hex;

/// Image vector table (IVT)
pub struct Ivt {
    /// IVT header
    pub header: Header,

    /// Address of the program entry point
    pub entry: u32,

    /// Address of the DCD, or `0` if unused
    pub dcd: u32,

    /// Address of the Boot Data section
    pub boot: u32,

    /// Absolute address of the boot image after it has been loaded
    pub self_: u32,
 
    /// Address of the Command Sequence File (CSF) section, or `0` if unused
    pub csf: u32,
}

impl fmt::Debug for Ivt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ivt")
            .field("header", &self.header)
            .field("self", &Hex(self.self_))
            .field("boot", &Hex(self.boot))
            .field("dcd", &Hex(self.dcd))
            .field("csf", &Hex(self.csf))
            .field("entry", &Hex(self.entry))
            .finish()
    }
}

impl Ivt {
    /// Size when binary encoded
    pub(crate) const SIZE: u8 = LENGTH as u8;

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(input)?;
        let (input, entry) = number::le_u32(input)?;
        let (input, _reserved1) = bytes::tag(&[0; 4])(input)?;
        let (input, dcd) = number::le_u32(input)?;
        let (input, boot) = number::le_u32(input)?;
        let (input, self_) = number::le_u32(input)?;
        let (input, csf) = number::le_u32(input)?;
        let (input, _reserved2) = bytes::tag(&[0; 4])(input)?;

        Ok((
            input,
            Self {
                header,
                entry,
                dcd,
                boot,
                self_,
                csf,
            },
        ))
    }
}

// expected values
const TAG: u8 = 0xD1;
const LENGTH: u16 = 32;
const VERSION0: u8 = 0x40;
const VERSION1: u8 = 0x41;

/// IVT header
#[derive(Debug)]
pub struct Header {
    /// Tag (always 0xD1)
    pub tag: u8,

    /// Length of the IVT header (always 32)
    pub length: u16,

    /// Version of the IVT header (0x40 or 0x41)
    pub version: u8,
}

impl Header {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, tag) = bytes::tag(&[TAG])(input)?;
        let (input, length) = number::be_u16(input)?;
        if length != LENGTH {
            todo!()
        }
        let (input, version) =
            branch::alt((bytes::tag(&[VERSION0]), bytes::tag(&[VERSION1])))(input)?;

        Ok((
            input,
            Self {
                tag: tag[0],
                length,
                version: version[0],
            },
        ))
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            tag: TAG,
            length: LENGTH,
            version: VERSION0,
        }
    }
}
