//! Sanity check the AES-128 API when paired with a user-provided RAM key
//!
//! Encryption has been checked against the following x86_64 Rust code
//!
//! ``` rust
//! use aesni::{block_cipher_trait::BlockCipher, Aes128}; // aesni = "0.6.0"
//!
//! let key = [185, /* .. */, 72];
//! let plaintext = [179, /* .. */, 242];
//! let mut block = plaintext.into();
//! let aes = Aes128::new(&key.into());
//! aes.encrypt_block(&mut block);
//! let ciphertext = block;
//! ```
//!
//! Expected output:
//!
//! ```
//! key:        [185, 105, 131, 156, 95, 42, 127, 229, 107, 50, 14, 134, 232, 8, 250, 72]
//! plaintext:  [179, 176, 19, 230, 198, 237, 169, 162, 83, 237, 103, 21, 175, 240, 64, 242]
//! ciphertext: [142, 123, 9, 120, 18, 121, 255, 69, 96, 169, 228, 53, 204, 12, 64, 21]
//! expected:   [142, 123, 9, 120, 18, 121, 255, 69, 96, 169, 228, 53, 204, 12, 64, 21]
//! encrypt: OK
//! decrypt: OK
//! ```

#![allow(dead_code)]
#![deny(unused_must_use)]
#![no_main]
#![no_std]

use block_cipher_trait::BlockCipher as _;
use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{dcp::Aes128, memlog, memlog_flush_and_reset};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let key = [
        185, 105, 131, 156, 95, 42, 127, 229, 107, 50, 14, 134, 232, 8, 250, 72,
    ];

    memlog!("key:        {:?}", key);

    let cipher = Aes128::new(&key.into());

    let plaintext = [
        179, 176, 19, 230, 198, 237, 169, 162, 83, 237, 103, 21, 175, 240, 64, 242,
    ];

    memlog!("plaintext:  {:?}", plaintext);

    let expected = [
        142, 123, 9, 120, 18, 121, 255, 69, 96, 169, 228, 53, 204, 12, 64, 21,
    ]
    .into();
    let mut block = plaintext.into();

    cipher.encrypt_block(&mut block);

    memlog!("ciphertext: {:?}", block);
    memlog!("expected:   {:?}", expected);

    if block == expected {
        memlog!("encrypt: OK");
    } else {
        memlog!("output didn't match the expected value");
        memlog_flush_and_reset!();
    }

    cipher.decrypt_block(&mut block);

    if block[..] == plaintext[..] {
        memlog!("decrypt: OK");
    } else {
        memlog!("decryption didn't return the input plaintext");
        memlog_flush_and_reset!();
    }

    memlog_flush_and_reset!();
}
