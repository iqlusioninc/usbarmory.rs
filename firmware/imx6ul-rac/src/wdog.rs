//! Watchdog

const WDOG1_BASE: usize = 0x020b_c000;

/// Watchdog Control Register
pub const WDOG1_WCR: *mut u16 = WDOG1_BASE as *mut _;

/// Software Reset Signal
pub const WDOG_WCR_SRS: u16 = 1 << 4;

/// Software Reset Extension
pub const WDOG_WCR_SRE: u16 = 1 << 6;
