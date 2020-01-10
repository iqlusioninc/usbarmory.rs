//! Global Interrupt Controller (GIC)

// Section 3.2 lists the Cortex A7 interrupts

// See section 2.2 of the RM
const GIC_BASE: usize = 0xa0_0000;

pub mod gicc;
pub mod gicd;
