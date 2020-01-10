#![no_main]
#![no_std]

use panic_serial as _;

#[no_mangle]
fn main() -> ! {
    // (arbitrarily long string to verify that the entire panic message is flushed before reset)
    panic!("Oh no! S0m3th1nG w3nt wr0ngZzz");
}
