// Generates an undefined instruction exception to test that overriding
// exception handlers work

#![no_main]
#![no_std]

use exception_reset as _;
use panic_serial as _;
use usbarmory::{println, serial::Serial};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    // this operation will trigger the `UndefinedInstruction` handler defined
    // below
    cortex_a::udf()
}

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn UndefinedInstruction() -> ! {
    println!("You've met with a terrible fate, haven't you?");

    // wait 5 seconds
    Serial::flush();

    usbarmory::reset()
}
