//! Watchdog

const WDOG1_BASE: usize = 0x020b_c000;

/// Watchdog Control Register
pub const WDOG1_WCR: *mut u32 = WDOG1_BASE as *mut u32;

/// Software Reset Extension
pub const WDOG_WCR_SRE: u32 = 1 << 6;
