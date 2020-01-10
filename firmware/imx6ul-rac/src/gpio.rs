//! General Purpose Input/Output (GPIO)

// See chapter 26 of the reference manual

const GPIO4_BASE: usize = 0x020A_8000;
const DR_OFFSET: usize = 0x0;
const DIR_OFFSET: usize = 0x4;

/// GPIO4 Data Register
pub const GPIO4_DR: *mut u32 = (GPIO4_BASE + DR_OFFSET) as *mut u32;

/// GPIO4 Direction Register
pub const GPIO4_DIR: *mut u32 = (GPIO4_BASE + DIR_OFFSET) as *mut u32;
