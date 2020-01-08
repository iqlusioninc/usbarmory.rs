//! Runtime crate for the USB Armory Mk II (i.MX6UL core)

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

// NOTE due to ABI requirements the real entry point, `_start`, is written in
// assembly and lives in the `asm.s` file. That subroutine calls this one.
// NOTE(C ABI) Rust ABI is unspecified / unstable; all calls between Rust code
// and external assembly must use the stable C ABI
#[no_mangle]
unsafe extern "C" fn start() -> ! {
    extern "Rust" {
        // NOTE(Rust ABI) this subroutine is provided by a Rust crate
        fn main() -> !;
    }

    // TODO RAM initialization

    main()
}
