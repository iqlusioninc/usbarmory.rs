//! Current Program Status Register

/// Reads the CPSR register
pub fn read() -> u32 {
    extern "C" {
        fn __cpsr_r() -> u32;
    }
    unsafe { __cpsr_r() }
}
