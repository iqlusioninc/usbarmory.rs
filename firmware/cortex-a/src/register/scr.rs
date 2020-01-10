//! Security Control Register

//! Reads the SCR register
pub fn read() -> u16 {
    let scr;
    unsafe {
        asm!("mrc p15, 0, $0, c1, c1, 0" : "=r"(scr));
    }
    scr
}
