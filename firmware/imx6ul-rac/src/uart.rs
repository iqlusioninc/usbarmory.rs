//! Universal Asynchronous Receiver/Transmitter (UART)

// See chapter 53 of the reference manual

const UART2_BASE: usize = 0x021e_8000;
const UTXD_OFFSET: usize = 0x40;
const USR1_OFFSET: usize = 0x94;
const USR2_OFFSET: usize = 0x98;

/// UART2 Transmitter Register
pub const UART2_UTXD: *mut u32 = (UART2_BASE + UTXD_OFFSET) as *mut _;

/// UART2 Status Register 1
pub const UART2_USR1: *mut u32 = (UART2_BASE + USR1_OFFSET) as *mut _;

/// UART2 Status Register 2
pub const UART2_USR2: *mut u32 = (UART2_BASE + USR2_OFFSET) as *mut _;

/// Transmitter Ready Interrupt
pub const UART_USR1_TRDY: u32 = 1 << 13;

/// Transmitter Buffer FIFO Empty
pub const UART_USR2_TXFE: u32 = 1 << 14;

/// Transmitter Complete
pub const UART_USR2_TXDC: u32 = 1 << 3;
