//! Random Number Generation
//!
//! Expected output:
//!
//! ```
//! first seed generated in 3.932952ms
//! Rng::next_u32 -> 3395530786
//! Rng::write -> [2290995670, 2824611633, 1576451376, 2540853980]
//! Stats {
//!     monobit_test_failed: false,
//!     length_1_run_test_failed: false,
//!     length_2_run_test_failed: false,
//!     length_3_run_test_failed: false,
//!     length_4_run_test_failed: false,
//!     length_5_run_test_failed: false,
//!     length_6_plus_run_test_failed: false,
//!     long_run_test: false,
//! }
//! ```
//!
//! But with different random numbers

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    println,
    rng::{self, Rng},
    serial::Serial,
    time::Instant,
};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let start = Instant::now();
    let rng = Rng::initialize().expect("UNREACHABLE");

    rng.wait_for_initial_seed();
    let elapsed = start.elapsed();

    println!("first seed generated in {:?}", elapsed);

    println!("Rng::next_u32 -> {}", rng.next_u32());

    let mut buf = [0; rng::FIFO_SIZE];
    println!("Rng::write -> {:?}", rng.write(&mut buf));

    println!("{:#?}", rng.stats());

    Serial::flush();

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
