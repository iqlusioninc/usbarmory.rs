//! Runtime crate for the USB Armory Mk II (i.MX6UL core)

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use pac::{gicc::GICC, gpio::GPIO4};

// Software Generated Interrupts
extern "C" {
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

static SGIS: [unsafe extern "C" fn(); 16] = [
    SGI0, SGI1, SGI2, SGI3, SGI4, SGI5, SGI6, SGI7, SGI8, SGI9, SGI10, SGI11, SGI12, SGI13, SGI14,
    SGI15,
];

include!(concat!(env!("OUT_DIR"), "/interrupts.rs"));

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
    // NOTE(borrow_unchecked) this is "before main"; no singletons exist yet
    GPIO4::borrow_unchecked(|gpio| {
        // set them as outputs
        let old = gpio.GDIR.read();
        gpio.GDIR.write(old | BLUE | WHITE);
        // turn the white LED on and the blue LED off to indicate we are alive
        let old = gpio.DR.read();
        gpio.DR.write((old | BLUE) & !WHITE);
    });

    extern "Rust" {
        // NOTE(Rust ABI) this subroutine is provided by a Rust crate
        fn main() -> !;
    }

    main()
}

#[no_mangle]
extern "C" fn IRQ() {
    // NOTE(borrow_unchecked) IRQs are masked, plus this is a single-instruction
    // read of a read-only register (that has side effects, though)
    let iar = GICC::borrow_unchecked(|gicc| gicc.IAR.read());

    let iid = (iar & ((1 << 10) - 1)) as u16;

    let f = if iid == 1023 {
        // spurious interrupt
        return;
    } else if iid < 16 {
        // Software Generated Interrupt
        SGIS[iid as usize]
    } else if iid < (32 + 128) {
        // Shared Peripheral Interrupt
        // NOTE(get_unchecked) avoid panicking branch
        unsafe { *SPIS.get_unchecked((iid - 32) as usize) }
    } else {
        extern "C" {
            fn DefaultHandler() -> !;
        }

        unsafe { DefaultHandler() }
    };

    unsafe {
        cortex_a::enable_irq();
        f();
    }
    cortex_a::disable_irq();

    // NOTE(borrow_unchecked) single-instruction write to a write-only register
    GICC::borrow_unchecked(|gicc| {
        // end of interrupt
        gicc.EOIR.write(iid as u32);
    });
}

// NOTE this is written in assembly because it should never touch the stack
// pointer regardless of the optimization level used to compile the application
// #[no_mangle]
// fn DataAbort() -> ! {
//     static MSG: &str = "\ndata abort exception (it could indicate a stack overflow)\n";
//     Serial::borrow_unchecked(|serial| {
//         serial.write_all(MSG.as_bytes());
//     });
//     Serial::flush();
//     usbarmory::reset();
// }
