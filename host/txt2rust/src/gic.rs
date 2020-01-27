//! GICv2
//!
//! # References
//!
//! - "Cortex-A7 TRM": Cortex-A7 Technical Reference Manual (DDI 0464D)
//! - "RM": i.MX 6UltraLite Applications Processor Reference Manual (IMX6ULRM)
//! - "GICv2": ARM Generic Interrupt Controller 2.0 Architecture Specification
//! (ARM IHI 0048B.b)

use crate::{
    compress::{Instances, Peripheral, Register},
    Access,
};

// There's no register table for the GIC in the RM; this information comes
// from the RM and the other references listed above

// See section 2.2 of the RM
const BASE: u32 = 0xa0_0000;

/// GIC CPU interface
pub fn gicc() -> Peripheral<'static> {
    // See section 8.2.1 of the Cortex-A7 TRM
    const OFFSET: u32 = 0x2000;

    // See section 4.1.3 of GICv2; that section also says "All GIC registers are
    // 32-bits wide"
    let width = 32;
    let registers = vec![
        Register {
            offset: 0x0,
            description: "CPU Interface Control Register".into(),
            name: "CTLR",
            width,
            reset_value: Some(0x0000_0000),
            access: Access::ReadWrite,
            instances: 1,
            unsafe_write: false,
        },
        Register {
            offset: 0x4,
            description: "Interrupt Priority Mask Register".into(),
            name: "PMR",
            width,
            reset_value: Some(0x0000_0000),
            access: Access::ReadWrite,
            instances: 1,
            // can be used to create critical sections; writes can break those
            // critical sections
            unsafe_write: true,
        },
        Register {
            offset: 0xc,
            description: "Interrupt Acknowledge Register".into(),
            name: "IAR",
            width,
            reset_value: Some(0x0000_03ff),
            access: Access::ReadOnly,
            instances: 1,
            unsafe_write: false,
        },
        Register {
            offset: 0x10,
            description: "End of Interrupt Register".into(),
            name: "EOIR",
            width,
            reset_value: None,
            access: Access::WriteOnly,
            instances: 1,
            unsafe_write: false,
        },
    ];

    Peripheral {
        name: "GICC",
        instances: Instances::Single(BASE + OFFSET),
        registers,
    }
}

/// GIC Distributor
pub fn gicd() -> Peripheral<'static> {
    // See section 8.2.1 of the Cortex-A7 TRM
    const OFFSET: u32 = 0x1000;
    // See section 1.4 of the RM
    const INTERRUPTS: u16 = 128;

    // See section 4.1.2 of GICv2; that section also says "All GIC registers are
    // 32-bits wide"
    let width = 32;
    let registers = vec![
        Register {
            offset: 0x0,
            description: "Distributor Control Register".into(),
            name: "CTLR",
            width,
            reset_value: Some(0x0000_0000),
            access: Access::ReadWrite,
            instances: 1,
            unsafe_write: false,
        },
        Register {
            offset: 0x100,
            description: "Interrupt Set-Enable Registers".into(),
            name: "ISENABLER",
            width,
            reset_value: None, // implementation defined
            access: Access::ReadWrite,
            // each bit of this register corresponds to an interrupt
            instances: INTERRUPTS / 32,
            // can be used to create critical sections; writes can break those
            // critical sections
            unsafe_write: true,
        },
        Register {
            offset: 0x400,
            description: "Interrupt Priority Registers".into(),
            name: "IPRIORITYR",
            // "These registers are byte-accessible" -- section 4.3.11 of GICv2
            width: 8,
            reset_value: None, // implementation defined
            access: Access::ReadWrite,
            instances: INTERRUPTS,
            // can be used to create critical sections; writes can break those
            // critical sections
            unsafe_write: true,
        },
        Register {
            offset: 0xf00,
            description: "Software Generated Interrupt Register".into(),
            name: "SGIR",
            width,
            reset_value: None,
            access: Access::WriteOnly,
            instances: 1,
            unsafe_write: false,
        },
    ];

    Peripheral {
        name: "GICD",
        instances: Instances::Single(BASE + OFFSET),
        registers,
    }
}
