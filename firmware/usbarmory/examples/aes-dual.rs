//! Use two ciphers from the same context

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

    memlog!("key:         {:?}", key);

    let cipher1 = Aes128::new(&key.into());
    let cipher2 = Aes128::new_unique().expect("HW key in use");

    // check that you can't get two instances of the HW AES
    assert!(
        Aes128::new_unique().is_none(),
        "error: was able to create a second HW AES"
    );

    let plaintext = [
        179, 176, 19, 230, 198, 237, 169, 162, 83, 237, 103, 21, 175, 240, 64, 242,
    ];

    memlog!("plaintext:   {:?}", plaintext);

    let mut ciphertext1 = plaintext.into();

    cipher1.encrypt_block(&mut ciphertext1);

    let mut ciphertext2 = ciphertext1.clone().into();

    cipher2.encrypt_block(&mut ciphertext2);

    memlog!("ciphertext1: {:?}", ciphertext1);
    memlog!("ciphertext2: {:?}", ciphertext2);

    // release the ciphers and re-create them
    drop(cipher1);
    drop(cipher2);
    let cipher1 = Aes128::new(&key.into());
    let cipher2 = Aes128::new_unique().expect("HW cipher in use");

    let mut ciphertext1d = ciphertext2.clone();

    cipher2.decrypt_block(&mut ciphertext1d);

    let mut plaintextd = ciphertext1d.clone();

    cipher1.decrypt_block(&mut plaintextd);

    memlog!("ciphertext1? {:?}", ciphertext1d);
    memlog!("plaintext?   {:?}", plaintext);

    if ciphertext1d != ciphertext1 || plaintextd[..] != plaintext[..] {
        memlog!("decryption error");
    } else {
        memlog!("decryption: OK");
    }

    memlog_flush_and_reset!();
}
