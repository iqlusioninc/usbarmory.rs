//! GIC Distributor

// See section 8.2.1 of the Cortex-A7 TRM (DDI 0464D)
const GICD_OFFSET: usize = 0x1000;

// See section 4.1.2 of the GIC 2.0 Architecture Specification (ID072613)
const CTLR_OFFSET: usize = 0x0;
const ISENABLER_OFFSET: usize = 0x100;
const IPRIORITYR_OFFSET: usize = 0x400;
const SGIR_OFFSET: usize = 0xf00;

/// Distributor Control Register
pub const GICD_CTLR: *mut u32 = (super::GIC_BASE + GICD_OFFSET + CTLR_OFFSET) as *mut u32;

/// Interrupt Set-Enable Registers
pub const GICD_ISENABLER: *mut u32 = (super::GIC_BASE + GICD_OFFSET + ISENABLER_OFFSET) as *mut u32;

/// Software Generated Interrupt Register
pub const GICD_SGIR: *mut u32 = (super::GIC_BASE + GICD_OFFSET + SGIR_OFFSET) as *mut u32;

/// Interrupt Priority Registers
pub const GICD_IPRIORITYR: *mut u8 = (super::GIC_BASE + GICD_OFFSET + IPRIORITYR_OFFSET) as *mut u8;
