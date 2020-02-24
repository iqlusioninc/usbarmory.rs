//! eMMC commands

use crate::emmc::Rca;

// Table 56-3 in ULRM
#[derive(Clone, Debug)]
pub enum Command {
    // No-response commands
    // 0
    GoIdleState,

    // R1 commands
    // 3
    SetRelativeAddr {
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

    // R1b commands
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

    // R2 commands
    // 2
    AllSendCid,
    // 9
    SendCsd {
        rca: Rca,
    },

    // R3 commands
    // 1
    SendOpCond {
        /// `u24` bits 8..=23 indicate the voltage range (2.0V -> 3.6V in 0.1V steps)
        /// A value of `0` will query the supported voltage range of all cards
        voltage_range: u32,
    },
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
            Command::Switch { .. } => 7,
            Command::SelectCard { .. } => 7,
            Command::SendCsd { .. } => 9,
            Command::SendStatus { .. } => 13,
            Command::SetBlockLen { .. } => 16,
            Command::ReadSingleBlock { .. } => 17,
            Command::WriteSingleBlock { .. } => 24,
        }
    }

    /// Extracts the argument of this command
    pub fn arg(&self) -> u32 {
        match self {
            Command::AllSendCid => 0,
            Command::GoIdleState => 0,
            Command::ReadSingleBlock { block_nr } => *block_nr,
            Command::SelectCard { rca } => rca.map(|rca| u32::from(rca.get())).unwrap_or(0) << 16,
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
    pub fn typ(&self) -> u8 {
        // Only CMD12 & CMD52 use a different value
        let idx = self.index();
        assert!(idx != 12 && idx != 52, "unimplemented");

        0b00
    }

    /// Whether this command uses any of the data lines
    // See table 56-3 of ULRM; `adtc` commands transfer data; R1b commands
    // indicate busy-ness on the DATA0 line
    pub fn uses_data_line(&self) -> bool {
        self.data_present()
            || match self {
                // R1b commands
                Command::Switch { .. } | Command::SelectCard { .. } => true,
                _ => false,
            }
    }

    /// Whether this command transfers data (in either direction)
    // See table 56-3 of ULRM; `adtc` commands transfer data
    pub fn data_present(&self) -> bool {
        match self {
            // adtc commands
            Command::ReadSingleBlock { .. } | Command::WriteSingleBlock { .. } => true,
            _ => false,
        }
    }

    /// The response type that this command expects
    // NOTE see tables 56-3 and 56-6 for a mapping from command to response type + checks
    pub fn response_type(&self) -> u8 {
        const NO_RESPONSE: u8 = 0b00;
        // 136-bit response
        const B136_RESPONSE: u8 = 0b01;
        // 48-bit response
        const B48_RESPONSE: u8 = 0b10;
        // 48-bit response, check Busy after response
        const B48_RESPONSE_CHECK_BUSY: u8 = 0b11;

        match self {
            // No response commands
            Command::GoIdleState => NO_RESPONSE,

            // R1 commands
            Command::SetRelativeAddr { .. }
            | Command::SendStatus { .. }
            | Command::SetBlockLen { .. }
            | Command::ReadSingleBlock { .. }
            | Command::WriteSingleBlock { .. } => B48_RESPONSE,

            // R1b commands
            Command::SelectCard { .. } | Command::Switch { .. } => B48_RESPONSE_CHECK_BUSY,

            // R2 commands
            Command::AllSendCid | Command::SendCsd { .. } => B136_RESPONSE,

            // R3 commands
            Command::SendOpCond { .. } => B48_RESPONSE,
        }
    }

    /// Check command index in the response
    pub fn cicen(&self) -> bool {
        match self {
            // No response commands
            Command::GoIdleState => false,

            // R1 commands
            Command::SetRelativeAddr { .. }
            | Command::SendStatus { .. }
            | Command::SetBlockLen { .. }
            | Command::ReadSingleBlock { .. }
            | Command::WriteSingleBlock { .. } => true,

            // R1b commands
            Command::SelectCard { .. } | Command::Switch { .. } => true,

            // R2 commands
            Command::AllSendCid | Command::SendCsd { .. } => false,

            // R3 commands
            Command::SendOpCond { .. } => false,
        }
    }

    /// Check command index in the response
    pub fn cccen(&self) -> bool {
        match self {
            // No response commands
            Command::GoIdleState => false,

            // R1 commands
            Command::SetRelativeAddr { .. }
            | Command::SendStatus { .. }
            | Command::SetBlockLen { .. }
            | Command::ReadSingleBlock { .. }
            | Command::WriteSingleBlock { .. } => true,

            // R1b commands
            Command::SelectCard { .. } | Command::Switch { .. } => true,

            // R2 commands
            Command::AllSendCid | Command::SendCsd { .. } => true,

            // R3 commands
            Command::SendOpCond { .. } => false,
        }
    }
}
