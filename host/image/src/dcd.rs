//! Device Configuration Data

use core::{convert::TryFrom, fmt};

use nom::{bits::complete as bits, bytes::complete as bytes, number::complete as number, IResult};

use crate::Hex;

/// Device Configuration Data
#[derive(Debug)]
pub struct Dcd {
    /// DCD header
    pub header: Header,

    /// DCD commands
    pub commands: Vec<Command>,
}

impl Dcd {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(input)?;
        let length = header
            .length
            .checked_sub(Header::SIZE.into())
            .expect("TODO");
        let (input, commands) = Command::parse(input, length)?;

        Ok((
            input,
            Self {
                header,
                commands: commands,
            },
        ))
    }
}

// expected values
const MAX_LENGTH: u16 = 1768;
const VERSION: u8 = 0x40;

/// DCD header
#[derive(Debug)]
pub struct Header {
    /// Tag (always `0xD2`)
    pub tag: u8,

    /// Length of the DCD (including the header, max value = 1768)
    pub length: u16,

    /// Version of the DCD (always `0x40`)
    pub version: u8,
}

impl Header {
    pub(crate) const SIZE: u8 = 4;
    const TAG: u8 = 0xD2;

    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, tag) = bytes::tag(&[Self::TAG])(input)?;
        let (input, length) = number::be_u16(input)?;
        if length > MAX_LENGTH {
            todo!("bubble up error")
        }
        let (input, version) = bytes::tag(&[VERSION])(input)?;

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
            tag: Self::TAG,
            length: 0,
            version: VERSION,
        }
    }
}

/// DCD commands
#[derive(Debug)]
pub enum Command {
    /// Write Data command
    WriteData(WriteDataCommand),
}

impl Command {
    fn parse(mut input_: &[u8], mut length: u16) -> IResult<&[u8], Vec<Command>> {
        let mut list = vec![];
        while length != 0 {
            let (input, cmd) = WriteDataCommand::parse(input_, &mut length)?;
            list.push(cmd);
            input_ = input;
        }
        Ok((input_, list))
    }
}

/// Write Data command
#[derive(Debug)]
pub struct WriteDataCommand {
    /// Command header
    pub header: WriteDataCommandHeader,

    /// List of address value pairs to write
    pub address_value: Vec<AddressValue>,
}

impl WriteDataCommand {
    fn parse<'i>(input: &'i [u8], length: &mut u16) -> IResult<&'i [u8], Command> {
        let (input, header) = WriteDataCommandHeader::parse(input)?;
        *length -= header.length;
        let (input, address_value) = AddressValue::parse(
            input,
            header
                .length
                .checked_sub(WriteDataCommandHeader::SIZE.into())
                .expect("TODO"),
            header.parameter.bytes,
        )?;
        Ok((
            input,
            Command::WriteData(WriteDataCommand {
                header,
                address_value,
            }),
        ))
    }

    pub(crate) fn writes(address_value: Vec<AddressValue>) -> Self {
        let length = u16::from(WriteDataCommandHeader::SIZE)
            + u16::try_from(8 * address_value.len()).expect("TODO");
        let header = WriteDataCommandHeader {
            tag: WriteDataCommandHeader::TAG,
            length,
            parameter: WriteDataCommandParameter {
                bytes: Bytes::B4,
                flags: Flags::WriteValue,
            },
        };

        Self {
            header,
            address_value,
        }
    }
}

/// Header of the Write Data command
#[derive(Debug)]
pub struct WriteDataCommandHeader {
    /// Tag (always 0xCC)
    pub tag: u8,

    /// Length of the Write Data command (including the header)
    pub length: u16,

    /// Command parameter
    pub parameter: WriteDataCommandParameter,
}

impl WriteDataCommandHeader {
    pub(crate) const SIZE: u8 = 4;
    const TAG: u8 = 0xCC;

    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, tag) = bytes::tag(&[Self::TAG])(input)?;
        let (input, length) = number::be_u16(input)?;
        let (input, parameter) = nom::bits::bits(WriteDataCommandParameter::parse)(input)?;

        Ok((
            input,
            Self {
                tag: tag[0],
                length,
                parameter,
            },
        ))
    }
}

/// Write Data command parameter
#[derive(Debug)]
pub struct WriteDataCommandParameter {
    /// Width of the target locations
    pub bytes: Bytes,

    /// Control flags
    pub flags: Flags,
}

impl WriteDataCommandParameter {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (input, flags) = Flags::parse(input)?;
        let (input, bytes) = Bytes::parse(input)?;
        Ok((input, Self { bytes, flags }))
    }
}

/// Width of the target locations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bytes {
    /// 1 byte
    B1 = 1,

    /// 2 bytes
    B2 = 2,

    /// 4 bytes
    B4 = 4,
}

impl Bytes {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (input, bits): (_, u8) = bits::take(3usize)(input)?;
        let bytes = if bits == 1 {
            Bytes::B1
        } else if bits == 2 {
            Bytes::B2
        } else if bits == 4 {
            Bytes::B4
        } else {
            panic!("TODO")
        };
        Ok((input, bytes))
    }
}

/// Control flags
#[derive(Clone, Copy, Debug)]
pub enum Flags {
    /// `*address = mask`
    WriteValue = 0b00,

    /// `*address &= !mask`
    ClearBitmask = 0b10,

    /// `*address |= mask`
    SetBitmask = 0b11,
}

impl Flags {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (input, _zeros): (_, u8) = bits::tag(0b000, 3usize)(input)?;
        let (input, flags): (_, u8) = bits::take(2usize)(input)?;
        let data_mask = flags & 0b01 != 0;
        let data_set = flags & 0b10 != 0;
        let flags = if data_mask {
            if data_set {
                Flags::SetBitmask
            } else {
                Flags::ClearBitmask
            }
        } else {
            Flags::WriteValue
        };
        Ok((input, flags))
    }
}

/// Address value pair
#[derive(Clone, Copy)]
pub struct AddressValue {
    /// Address to write
    pub address: u32,

    /// Value to write
    pub value: u32,
}

impl AddressValue {
    fn parse(mut input_: &[u8], length: u16, bytes: Bytes) -> IResult<&[u8], Vec<Self>> {
        let mut list = vec![];
        for _ in 0..(length / (4 + bytes as u16)) {
            let (input, address) = number::be_u32(input_)?;
            let (input, value) = match bytes {
                Bytes::B1 => number::be_u8(input).map(|(i, v)| (i, v.into()))?,
                Bytes::B2 => number::be_u16(input).map(|(i, v)| (i, v.into()))?,
                Bytes::B4 => number::be_u32(input)?,
            };
            list.push(AddressValue { address, value });
            input_ = input;
        }
        Ok((input_, list))
    }
}

impl fmt::Debug for AddressValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddressValue")
            .field("address", &Hex(self.address))
            .field("value", &Hex(self.value))
            .finish()
    }
}
