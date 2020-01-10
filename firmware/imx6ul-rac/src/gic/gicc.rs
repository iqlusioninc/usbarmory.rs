//! GIC CPU interface

// See section 8.2.1 of the Cortex-A7 TRM (DDI 0464D)
const GICC_OFFSET: usize = 0x2000;

// See section 4.1.3 of the GIC 2.0 Architecture Specification (ID072613)
const CTLR_OFFSET: usize = 0x0;
const PMR_OFFSET: usize = 0x4;
const IAR_OFFSET: usize = 0xc;

/// CPU Interface Control Register
pub const GICC_CTLR: *mut u32 = (super::GIC_BASE + GICC_OFFSET + CTLR_OFFSET) as *mut _;

/// Priority Mask Register
pub const GICC_PMR: *mut u32 = (super::GIC_BASE + GICC_OFFSET + PMR_OFFSET) as *mut _;

/// Interrupt Acknowledge Register
pub const GICC_IAR: *mut u32 = (super::GIC_BASE + GICC_OFFSET + IAR_OFFSET) as *mut _;
