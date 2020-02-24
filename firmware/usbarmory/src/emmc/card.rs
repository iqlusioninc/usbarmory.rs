// References
// - (JESD84-A441) eMMC product standard (MMCA, 4.41)

/// Card Specific Data
#[derive(Debug)]
pub struct Csd {
    read_block_size_log2: u8,
    size: u16,
    size_mult: u8,
    spec_vers: u8,
    write_block_size_log2: u8,
}

impl Csd {
    /// Returns the version of the eMMC specification supported by the card
    pub fn version(&self) -> u8 {
        self.spec_vers
    }

    /// Maximum supported block length for read operations
    pub fn max_read_block_size(&self) -> u16 {
        1 << self.write_block_size_log2
    }

    /// Maximum supported block length for write operations
    pub fn max_write_block_size(&self) -> u16 {
        1 << self.write_block_size_log2
    }

    /// The number of memory blocks this card has
    ///
    /// NOTE for card with capacities greater than 2GB this value may not match
    /// the actual capacity of the card and the EXT_CSD register should be
    /// consulted.
    pub fn number_of_blocks(&self) -> u32 {
        (1 << (self.size_mult + 2)) * u32::from(self.size + 1)
    }
}

impl From<[u32; 4]> for Csd {
    fn from(rsps: [u32; 4]) -> Self {
        let extract = |offset, mask| {
            // RSP0[0] is CSD[8]
            const OFFSET: usize = 8;
            let offset = offset - OFFSET;

            (rsps[offset / 32] >> (offset % 32)) & mask
        };

        let spec_vers = extract(122, 0b1111) as u8;
        let write_block_size_log2 = extract(22, 0b1111) as u8;
        let read_block_size_log2 = extract(80, 0b1111) as u8;
        let size = extract(62, (1 << 12) - 1) as u16;
        let size_mult = extract(47, 0b111) as u8;

        Csd {
            read_block_size_log2,
            size,
            size_mult,
            spec_vers,
            write_block_size_log2,
        }
    }
}

/// Card status
#[derive(Debug)]
pub struct Status {
    pub ready_for_data: bool,
    pub state: State,
}

/// Card state
#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Idle,
    Ready,
    Identification,
    Standby,
    Transfer,
    SendingData,
    ReceiveData,
    Programming,
    Other,
}

impl From<u32> for Status {
    fn from(bits: u32) -> Self {
        const READY_FOR_DATA: u32 = 1 << 8;
        let ready_for_data = bits & READY_FOR_DATA != 0;
        let state = match (bits >> 9) & 0b1111 {
            0 => State::Idle,
            1 => State::Ready,
            2 => State::Identification,
            3 => State::Standby,
            4 => State::Transfer,
            5 => State::SendingData,
            6 => State::ReceiveData,
            7 => State::Programming,
            _ => State::Other,
        };

        Self {
            ready_for_data,
            state,
        }
    }
}
