//! System Control Register

/// Reads the SCTLR register
#[cfg(TODO = "external-assembly")]
pub fn read() -> u32 {
    let sctlr;
    unsafe { asm!("mrc p15, 0, $0, c1, c1, 0" : "=r"(sctlr)) }
    sctlr
}
