use core::fmt;

// See table 54-62 of the ULRM
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Token {
    inner: u32,
}

impl Token {
    pub const fn empty() -> Self {
        Token { inner: 0 }
    }

    pub fn set_total_bytes(&mut self, n: usize) {
        assert!(n < 0x5000);

        self.inner &= !(0xffff << 16);
        self.inner |= (n as u32) << 16;
    }

    pub fn get_total_bytes(self) -> u16 {
        (self.inner >> 16) as u16
    }

    /// Enables interrupts on complete
    pub fn set_ioc(&mut self) {
        const IOC: u32 = 1 << 15;
        self.inner |= IOC;
    }

    pub fn get_status(self) -> Status {
        Status {
            inner: self.inner as u8,
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.inner &= !0xff;
        self.inner |= u32::from(status.inner);
    }

    pub fn clear_status(&mut self) {
        self.inner &= !0xff
    }
}

impl From<u32> for Token {
    fn from(bits: u32) -> Self {
        Token { inner: bits }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("total_bytes", &self.get_total_bytes())
            .field("status", &self.get_status())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct Status {
    inner: u8,
}

impl From<u8> for Status {
    fn from(bits: u8) -> Self {
        Self { inner: bits }
    }
}

const STATUS_ACTIVE: u8 = 1 << 7;

impl Status {
    pub fn active() -> Self {
        Self {
            inner: STATUS_ACTIVE,
        }
    }

    pub fn is_active(self) -> bool {
        self.inner & STATUS_ACTIVE != 0
    }

    pub fn is_halted(self) -> bool {
        self.inner & (1 << 6) != 0
    }

    pub fn has_data_buffer_error(self) -> bool {
        self.inner & (1 << 5) != 0
    }

    pub fn has_transaction_error(self) -> bool {
        self.inner & (1 << 3) != 0
    }

    pub fn has_errors(self) -> bool {
        self.has_data_buffer_error() || self.has_transaction_error()
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Status")
            .field("active", &self.is_active())
            .field("halted", &self.is_halted())
            .field("data_buffer_error", &self.has_data_buffer_error())
            .field("transaction_error", &self.has_transaction_error())
            .finish()
    }
}
