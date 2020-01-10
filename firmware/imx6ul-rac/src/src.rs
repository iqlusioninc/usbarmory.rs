//! System Reset Controller

// See chapter 49 of the reference manual

const SRC_BASE: usize = 0x020d_8000;

/// SRC Control Register
pub const SRC_SCR: *mut u32 = SRC_BASE as *mut u32;

/// Software reset for debug of arm platform only
pub const SRC_SCR_CORES_DBG_RST_MASK: u32 = 1 << 21;

/// Software reset for core0 only
pub const SRC_SCR_CORE0_RST_MASK: u32 = 1 << 13;
