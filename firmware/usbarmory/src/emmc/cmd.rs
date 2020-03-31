//! eMMC commands

use crate::emmc::Rca;

// Table 56-3 in ULRM
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Command {
    // 0
    GoIdleState,

    // 1
    SendOpCond {
        /// `u24` bits 8..=23 indicate the voltage range (2.0V -> 3.6V in 0.1V steps)
        /// A value of `0` will query the supported voltage range of all cards
        ocr: u32,
    },

    // 2
    AllSendCid,

    // 3
    SetRelativeAddr {
        rca: Rca,
    },

    // 6
    // NOTE MMC specific version; `SWITCH_FUNC` is the SD variant of the command
    Switch {
        data: u32,
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
pub enum Response {
    None,
    R1,
    R1b,
    R2,
    R3,
    R4,
    R5,
    // doesn't exist in the standard
    // R5b,
    R6,
}

impl Command {
    /// The index of this command
    // NOTE returns a `u6`
    pub fn index(&self) -> u8 {
        self.index_type_arg_resp().0
    }

    // tables 22-25 of JESD84-A441
    fn index_type_arg_resp(&self) -> (/* index */ u8, Type, /* argument */ u32, Response) {
        const OCR_BUSY: u32 = 1 << 31;

        match self {
            // table 22
            Command::GoIdleState => (0, Type::bc, 0, Response::None),

            Command::SendOpCond { ocr } => (1, Type::bcr, ocr & !OCR_BUSY, Response::R3),

            Command::AllSendCid => (2, Type::bcr, 0, Response::R2),

            Command::SetRelativeAddr { rca } => {
                (3, Type::ac, u32::from(rca.get()) << 16, Response::R1)
            }

            Command::Switch { data } => (6, Type::ac, *data, Response::R1b),

            Command::SelectCard { rca } => (
                7,
                Type::ac,
                u32::from(rca.map(|nz| nz.get()).unwrap_or(0)) << 16,
                // NOTE transition from Stand-By state to Transfer state = R1
                // transition from Disconnected state to Programming state = R1b (unused)
                Response::R1,
            ),

            Command::SendExtCsd => (8, Type::adtc, 0, Response::R1),

            Command::SendCsd { rca } => (9, Type::ac, u32::from(rca.get()) << 16, Response::R2),

            Command::SendStatus { rca } => (13, Type::ac, u32::from(rca.get()) << 16, Response::R1),

            // table 23
            Command::SetBlockLen { len } => (16, Type::ac, *len, Response::R1),

            Command::ReadSingleBlock { block_nr } => (17, Type::adtc, *block_nr, Response::R1),

            // table 25
            Command::WriteSingleBlock { block_nr } => (24, Type::adtc, *block_nr, Response::R1),
        }
    }

    pub fn response(&self) -> Response {
        self.index_type_arg_resp().3
    }

    fn type_(&self) -> Type {
        self.index_type_arg_resp().1
    }

    /// Extracts the argument of this command
    pub fn arg(&self) -> u32 {
        self.index_type_arg_resp().2
    }

    // NOTE returns a `u2`
    pub fn cmdtyp(&self) -> u8 {
        // Only CMD12 & CMD52 use a different value
        let idx = self.index();
        assert!(idx != 52, "unimplemented");

        if idx == 12 {
            0b11
        } else {
            0b00
        }
    }

    /// Whether this command uses any of the data lines
    // See table 56-3 of ULRM; `adtc` commands transfer data; R1b commands
    // indicate busy-ness on the DATA0 line
    #[allow(dead_code)]
    pub fn uses_data_line(&self) -> bool {
        let resp = self.response();
        self.data_present() || resp == Response::R1b
    }

    /// Whether this command transfers data (in either direction)
    // See table 56-3 of ULRM; `adtc` commands transfer data
    pub fn data_present(&self) -> bool {
        self.type_() == Type::adtc
    }

    // See table 56-6 of ULRM
    fn resp_cicen_cccen(
        &self,
    ) -> (
        /* response_type: u8 */ u8,
        /* cicen: */ u8,
        /* cccen: */ u8,
    ) {
        match self.response() {
            Response::None => (0b00, 0, 0),
            Response::R2 => (0b01, 0, 1),
            Response::R3 | Response::R4 => (0b10, 0, 0),
            Response::R1 | Response::R5 | Response::R6 => (0b10, 1, 1),
            Response::R1b => (0b11, 1, 1),
        }
    }

    /// The response type that this command expects
    pub fn response_type(&self) -> u8 {
        self.resp_cicen_cccen().0
    }

    /// Check command index in the response
    pub fn cicen(&self) -> bool {
        self.resp_cicen_cccen().1 == 1
    }

    /// Check command CRC in the response
    pub fn cccen(&self) -> bool {
        self.resp_cicen_cccen().2 == 1
    }
}
