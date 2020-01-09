#![no_main]
#![no_std]

use panic_halt as _;
use usbarmory as _; // memory layout

#[no_mangle]
unsafe fn main() -> ! {
    const GPIO4: usize = 0x020A_8000;
    // Data Regsiter
    const GPIO4_DR: *mut u32 = (GPIO4 + 0) as *mut u32;
    // DIRection register
    const GPIO4_DIR: *mut u32 = (GPIO4 + 4) as *mut u32;

    const BLUE_MASK: u32 = 1 << 21;
    const WHITE_MASK: u32 = 1 << 22;

    // configure pins as output
    GPIO4_DIR.write_volatile(BLUE_MASK | WHITE_MASK);

    // turn the blue LED on; turn the white LED off
    let old = GPIO4_DR.read_volatile();
    GPIO4_DR.write_volatile((old | BLUE_MASK) & !WHITE_MASK);

    loop {}
}
