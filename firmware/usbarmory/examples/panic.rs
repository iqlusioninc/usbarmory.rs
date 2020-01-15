#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    // (arbitrarily long string to verify that the entire panic message is flushed before reset)
    panic!("Oh no! S0m3th1nG w3nt wr0ngZzz");
}
