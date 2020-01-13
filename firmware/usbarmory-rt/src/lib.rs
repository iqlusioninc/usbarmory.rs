//! Runtime crate for the USB Armory Mk II (i.MX6UL core)

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use rac::{gic::gicc, gpio};

// NOTE due to ABI requirements the real entry point, `_start`, is written in
// assembly and lives in the `asm.s` file. That subroutine calls this one.
// NOTE(C ABI) Rust ABI is unspecified / unstable; all calls between Rust code
// and external assembly must use the stable C ABI
#[no_mangle]
unsafe extern "C" fn start() -> ! {
    // NOTE RAM initialization is skipped here because u-boot takes care of it

    /* Initialize some peripherals that will always be configured in this way */
    // LEDS
    const BLUE: u32 = 1 << 22;
    const WHITE: u32 = 1 << 21;
    // set them as outputs
    let old = gpio::GPIO4_DIR.read_volatile();
    gpio::GPIO4_DIR.write_volatile(old | BLUE | WHITE);
    // turn the white LED on and the blue LED off to indicate we are alive
    let old = gpio::GPIO4_DR.read_volatile();
    gpio::GPIO4_DR.write_volatile((old | BLUE) & !WHITE);

    extern "Rust" {
        // NOTE(Rust ABI) this subroutine is provided by a Rust crate
        fn main() -> !;
    }

    main()
}

#[no_mangle]
unsafe extern "C" fn IRQ() {
    // acknowledge interrupt
    let iar = gicc::GICC_IAR.read_volatile();

    cortex_a::enable_irq();

    let iid = iar & ((1 << 10) - 1);

    if iid == 1023 {
        // spurious interrupt
        return;
    } else if iid < 16 {
        // Software Generated Interrupts
        extern "Rust" {
            fn SGI0();
            fn SGI1();
            fn SGI2();
            fn SGI3();
            fn SGI4();
            fn SGI5();
            fn SGI6();
            fn SGI7();
            fn SGI8();
            fn SGI9();
            fn SGI10();
            fn SGI11();
            fn SGI12();
            fn SGI13();
            fn SGI14();
            fn SGI15();
        }

        ([
            SGI0, SGI1, SGI2, SGI3, SGI4, SGI5, SGI6, SGI7, SGI8, SGI9, SGI10, SGI11, SGI12, SGI13,
            SGI14, SGI15,
        ][iid as usize])()
    } else {
        // TODO PPI (Private Peripheral Interrupt) / SPI (Shared PI)
        extern "C" {
            fn DefaultHandler();
        }

        cortex_a::disable_irq();
        DefaultHandler()
    }

    cortex_a::disable_irq();

    // end of interrupt
    gicc::GICC_EOIR.write_volatile(iid);
}
