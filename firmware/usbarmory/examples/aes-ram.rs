//! Sanity check the AES-128 API when paired with a user-provided RAM key
//!
//! The encryption is equivalent to the following Python code:
//!
//! ``` python
//! >>> from Crypto.Cipher import AES # pycryptodome 3.9.7
//! >>> key = bytearray([185, ... , 72])
//! >>> cipher = AES.new(key, AES.MODE_ECB)
//! >>> plaintext = bytearray([179, ... , 242])
//! >>> ciphertext = list(cipher.encrypt(plaintext)) # block, after encrypt_block
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
