// Generates a data abort exception to test that overriding exception handlers work

#![no_main]
#![no_std]

use exception_reset as _;
use panic_serial as _;
use usbarmory::println;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    // unaligned memory access = data abort exception
    unsafe {
        // this operation will trigger the `DataAbort` handler defined below
        (1 as *const u16).read_volatile();
    }

    usbarmory::reset()
}

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn DataAbort() -> ! {
    println!("You've met with a terrible fate, haven't you?");

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    usbarmory::reset()
}
