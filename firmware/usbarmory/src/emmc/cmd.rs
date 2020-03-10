//! eMMC commands

use crate::emmc::Rca;

// Table 56-3 in ULRM
#[derive(Clone, Debug)]
pub enum Command {
    // 0
    GoIdleState,

    // 1
    SendOpCond {
        /// `u24` bits 8..=23 indicate the voltage range (2.0V -> 3.6V in 0.1V steps)
        /// A value of `0` will query the supported voltage range of all cards
        voltage_range: u32,
    },

    // 2
    AllSendCid,

    // 3
    SetRelativeAddr {
        rca: Rca,
    },

    // 6
    // NOTE MMC specific version; `SWITCH_FUNC` is the SD variant of the command
    #[allow(dead_code)] // will be used to change the speed and data width
    Switch {
        /// `u2`
        access: u8,
        index: u8,
        value: u8,
        /// `u3`
        cmd: u8,
    },

    // 7
    SelectCard {
        rca: Option<Rca>,
    },

    // 8
    SendExtCsd,

    // 9
    SendCsd {
        rca: Rca,
    },

    // 13
    SendStatus {
        rca: Rca,
    },
    // 16
    SetBlockLen {
        len: u32,
    },

    // 17
    ReadSingleBlock {
        /// *Block* number (multiply by 512 to get the actual address)
        // NOTE for low capacity (<2GB) cards this is the actual 32-bit address
        block_nr: u32,
    },

    // 24
    WriteSingleBlock {
        /// *Block* address (multiply by 512 to get the actual address)
        // NOTE for low capacity (<2GB) cards this is the actual 32-bit address
        block_nr: u32,
    },
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
enum Type {
    /// Brodcast Command with no response
    bc,
    /// Broadcast Command with Response
    bcr,
    /// Addressed Command
    ac,
    /// Addressed Data Transfer Command
    adtc,
}

#[derive(PartialEq)]
enum Response {
    NoResponse,
    R1,
    R1b,
    R2,
    R3,
    R4,
    R5,
    R5b,
    R6,
}

impl Command {
    /// The index of this command
    // NOTE returns a `u6`
    pub fn index(&self) -> u8 {
        match self {
            Command::GoIdleState => 0,
            Command::SendOpCond { .. } => 1,
            Command::AllSendCid => 2,
            Command::SetRelativeAddr { .. } => 3,
            Command::Switch { .. } => 6,
            Command::SelectCard { .. } => 7,
            Command::SendExtCsd { .. } => 8,
            Command::SendCsd { .. } => 9,
            Command::SendStatus { .. } => 13,
            Command::SetBlockLen { .. } => 16,
            Command::ReadSingleBlock { .. } => 17,
            Command::WriteSingleBlock { .. } => 24,
        }
    }

    // see table 56-3
    fn response(&self) -> Response {
        match self {
            Command::GoIdleState => Response::NoResponse,
            Command::SendOpCond { .. } => Response::R3,
            Command::AllSendCid => Response::R2,
            Command::SetRelativeAddr { .. } => Response::R1, // eMMC (SDIO uses R6)
            Command::Switch { .. } => Response::R1b,         // eMMC (SDIO uses R1)
            Command::SendExtCsd => Response::R1,
            Command::SelectCard { .. } => Response::R1b,
            Command::SendCsd { .. } => Response::R2,
            Command::SendStatus { .. } => Response::R1,
            Command::SetBlockLen { .. } => Response::R1,
            Command::ReadSingleBlock { .. } => Response::R1,
            Command::WriteSingleBlock { .. } => Response::R1,
        }
    }

    fn type_(&self) -> Type {
        match self {
            Command::GoIdleState => Type::bcr,
            Command::SendOpCond { .. } => Type::bcr,
            Command::AllSendCid => Type::bcr,
            Command::SetRelativeAddr { .. } => Type::ac,
            Command::Switch { .. } => Type::ac, // eMMC (SDIO version is adtc)
            Command::SendExtCsd => Type::adtc,
            Command::SelectCard { .. } => Type::ac,
            Command::SendCsd { .. } => Type::ac,
            Command::SendStatus { .. } => Type::ac,
            Command::SetBlockLen { .. } => Type::ac,
            Command::ReadSingleBlock { .. } => Type::adtc,
            Command::WriteSingleBlock { .. } => Type::adtc,
        }
    }

    /// Extracts the argument of this command
    pub fn arg(&self) -> u32 {
        match self {
            Command::AllSendCid => 0,
            Command::GoIdleState => 0,
            Command::ReadSingleBlock { block_nr } => *block_nr,
            Command::SelectCard { rca } => rca.map(|rca| u32::from(rca.get())).unwrap_or(0) << 16,
            Command::SendExtCsd => 0,
            Command::SendCsd { rca } => u32::from(rca.get()) << 16,
            Command::SendOpCond { voltage_range } => *voltage_range,
            Command::SendStatus { rca } => u32::from(rca.get()) << 16,
            Command::SetBlockLen { len } => *len,
            Command::SetRelativeAddr { rca } => u32::from(rca.get()) << 16,
            Command::Switch {
                access,
                index,
                value,
                cmd,
            } => {
                (u32::from(*access) << 24)
                    | (u32::from(*index) << 16)
                    | (u32::from(*value) << 8)
                    | u32::from(*cmd)
            }
            Command::WriteSingleBlock { block_nr } => *block_nr,
        }
    }

    // NOTE returns a `u2`
    pub fn cmdtyp(&self) -> u8 {
        // Only CMD12 & CMD52 use a different value
        let idx = self.index();
        assert!(idx != 12 && idx != 52, "unimplemented");

        0b00
    }

    /// Whether this command uses any of the data lines
    // See table 56-3 of ULRM; `adtc` commands transfer data; R1b commands
    // indicate busy-ness on the DATA0 line
    pub fn uses_data_line(&self) -> bool {
        let resp = self.response();
        self.data_present() || resp == Response::R1b || resp == Response::R5b
    }

    /// Whether this command transfers data (in either direction)
    // See table 56-3 of ULRM; `adtc` commands transfer data
    pub fn data_present(&self) -> bool {
        self.type_() == Type::adtc
    }

    /// The response type that this command expects
    // NOTE see table 56-6
    pub fn response_type(&self) -> u8 {
        const NO_RESPONSE: u8 = 0b00;
        // 136-bit response
        const B136_RESPONSE: u8 = 0b01;
        // 48-bit response
        const B48_RESPONSE: u8 = 0b10;
        // 48-bit response, check Busy after response
        const B48_RESPONSE_CHECK_BUSY: u8 = 0b11;

        match self.response() {
            Response::NoResponse => NO_RESPONSE,
            Response::R2 => B136_RESPONSE,
            Response::R3 | Response::R4 => B48_RESPONSE,
            Response::R1 | Response::R5 | Response::R6 => B48_RESPONSE,
            Response::R1b | Response::R5b => B48_RESPONSE_CHECK_BUSY,
        }
    }

    /// Check command index in the response
    // see table 56-6
    pub fn cicen(&self) -> bool {
        match self.response() {
            Response::NoResponse => false,
            Response::R2 => false,
            Response::R3 | Response::R4 => false,
            Response::R1 | Response::R5 | Response::R6 => true,
            Response::R1b | Response::R5b => true,
        }
    }

    /// Check command CRC in the response
    pub fn cccen(&self) -> bool {
        match self.response() {
            Response::NoResponse => false,
            Response::R2 => true,
            Response::R3 | Response::R4 => false,
            Response::R1 | Response::R5 | Response::R6 => true,
            Response::R1b | Response::R5b => true,
        }
    }
}
