// References
// - (JESD84-A441) eMMC product standard (MMCA, 4.41)

use core::fmt;

/// Card Specific Data
#[derive(Debug)]
pub struct Csd {
    read_block_size_log2: u8,
    size: u16,
    size_mult: u8,
    spec_vers: u8,
    write_block_size_log2: u8,
}

#[cfg(TODO = "eMMC_and_uSD_support")]
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
    #[allow(dead_code)]
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

macro_rules! status {
	  ($($error:ident : $epos:expr),+;$($status:ident : $spos:expr),+) => {
        /// Card status
        #[derive(Clone, Copy)]
        pub struct Status {
            pub(crate) bits: u32,
            pub state: State,
            $(pub $error: bool,)+
            $(pub $status: bool,)+
        }

        impl fmt::Debug for Status {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut s = f.debug_struct("Status");
                s.field("State", &self.state);
                $(if self.$error {
                    s.field(stringify!($error), &true);
                })+
                $(if self.$status {
                    s.field(stringify!($status), &true);
                })+
                s.finish()
            }
        }

        impl Status {
            pub fn from(bits: u32) -> Result<Self, Self> {
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

                let error_mask = $(1 << $epos)|*;
                let has_errors = bits & error_mask != 0;

                $(let $error = bits & (1 << $epos) != 0;)+
                $(let $status = bits & (1 << $spos) != 0;)+
                let status = Self {
                    bits,
                    state,
                    $($error,)+
                    $($status,)+
                };

                if has_errors {
                    Err(status)
                } else {
                    Ok(status)
                }
            }
        }
	  };
}

impl PartialEq for Status {
    fn eq(&self, rhs: &Self) -> bool {
        self.bits == rhs.bits
    }
}

status!(
    address_out_of_range: 31,
    address_misalign: 30,
    block_len_error: 29,
    erase_seq_error: 28,
    erase_param: 27,
    wp_violation: 26,
    lock_unlock_failed: 24,
    com_crc_error: 23,
    illegal_command: 22,
    card_ecc_failed: 21,
    cc_error: 20,
    error: 19,
    underrun: 18,
    overrun: 17,
    cid_csd_overwrite: 16,
    wp_erase_skip: 15,
    erase_reset: 13,
    switch_error: 7;

    card_is_locked: 25,
    ready_for_data: 8,
    urgent_bkops: 6,
    app_cmd: 5
);

/// Card state
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
