//! Test our `Sha256` implementation against the `sha2` implementation

#![deny(warnings)]
#![no_main]
#![no_std]

use digest::{FixedOutput, Input, Reset};
use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{dcp::Sha256, memlog, memlog_flush_and_reset};

// The `expected` values come from running the following code on a x86_64 machine
//
// ``` rust
// use sha2::{Digest, Sha256};
// let input: &[u8] = /* .. */;
// let mut hasher = Sha256::new();
// hasher.input(input);
// let expected = hasher.result();
// ```
static TESTS: &[(/*input: */ &[u8], /* expected: */ [u8; 32])] = &[
    // 1 block
    (
        &[0; 64],
        [
            245, 165, 253, 66, 209, 106, 32, 48, 39, 152, 239, 110, 211, 9, 151, 155, 67, 0, 61,
            35, 32, 217, 240, 232, 234, 152, 49, 169, 39, 89, 251, 75,
        ],
    ),
    (
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
        ],
        [
            253, 234, 185, 172, 243, 113, 3, 98, 189, 38, 88, 205, 201, 162, 158, 143, 156, 117,
            127, 207, 152, 17, 96, 58, 140, 68, 124, 209, 217, 21, 17, 8,
        ],
    ),
    // 2 blocks
    (
        &[0; 128],
        [
            56, 114, 58, 46, 94, 138, 23, 170, 121, 80, 220, 0, 130, 9, 148, 78, 137, 143, 105,
            167, 189, 16, 162, 60, 131, 157, 52, 30, 147, 95, 213, 202,
        ],
    ),
    // partial block
    (
        &[0; 31],
        [
            253, 8, 190, 149, 123, 218, 7, 220, 82, 154, 216, 16, 13, 247, 50, 249, 206, 18, 174,
            62, 66, 188, 218, 106, 202, 190, 18, 192, 45, 253, 105, 137,
        ],
    ),
    (
        &[0; 63],
        [
            199, 114, 63, 161, 224, 18, 121, 117, 228, 158, 98, 231, 83, 219, 83, 146, 76, 27, 216,
            75, 138, 193, 172, 8, 223, 120, 208, 146, 112, 243, 217, 113,
        ],
    ),
];

// the input will be fed in chunks to the hasher -- the following chunk sizes will be used
const CHUNK_SIZES: &[usize] = &[128, 64, 32, 16, 11, 7, 5];

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    for (input, expected) in TESTS {
        if input.iter().all(|b| *b == 0) {
            memlog!("input:    {} zeros", input.len());
            usbarmory::memlog_try_flush();
        } else {
            memlog!("input:    {:?}", &input[..]);
            usbarmory::memlog_try_flush();
        }

        let mut first_hash = true;
        for chunk_size in CHUNK_SIZES {
            let mut hasher = Sha256::take().expect("taken");
            for chunk in input.chunks(*chunk_size) {
                hasher.input(chunk);
            }
            let output = hasher.fixed_result();

            if first_hash {
                memlog!("output:   {:?}", output);
                memlog!("expected: {:?}", expected);
                usbarmory::memlog_try_flush();
                first_hash = false;
            }

            if output[..] != expected[..] {
                memlog!("error: incorrect result with chunk_size={}", chunk_size);
                memlog_flush_and_reset!()
            }

            // test the `reset` API
            // hash
            let mut hasher = Sha256::take().expect("taken");
            for chunk in input.chunks(*chunk_size) {
                hasher.input(chunk);
            }
            // abort before result
            hasher.reset();

            // hash for real this time
            for chunk in input.chunks(*chunk_size) {
                hasher.input(chunk);
            }
            let output = hasher.fixed_result();

            if output[..] != expected[..] {
                memlog!(
                    "error: incorrect result with chunk_size={} & reset at the end",
                    chunk_size
                );
                memlog_flush_and_reset!()
            }

            // hash and immediately abort
            let mut hasher = Sha256::take().expect("taken");
            for chunk in input.chunks(*chunk_size) {
                hasher.input(chunk);
                hasher.reset();
                break;
            }

            // hash for real this time
            for chunk in input.chunks(*chunk_size) {
                hasher.input(chunk);
            }
            let output = hasher.fixed_result();

            if output[..] != expected[..] {
                memlog!(
                    "error: incorrect result with chunk_size={} & reset at the beginning",
                    chunk_size
                );
                memlog_flush_and_reset!()
            }
        }

        memlog!("OK");

        // flush a bit so the memlog buffer doesn't completely fill
        for _ in 0..100_000 {
            usbarmory::memlog_try_flush();
        }
    }

    memlog!("DONE");
    memlog_flush_and_reset!()
}
