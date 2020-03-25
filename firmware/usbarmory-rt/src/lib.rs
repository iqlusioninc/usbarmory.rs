//! Runtime crate for the USB Armory Mk II (i.MX6UL core)

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use core::sync::atomic::{self, Ordering};

use pac::GICC;

mod i2c;
mod leds;
mod rtc;
mod serial;

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

#[cfg(not(any(feature = "dram", feature = "ocram")))]
compile_error!("one of the following Cargo features must be enabled: `dram` or `ocram`");

#[cfg(all(feature = "dram", feature = "ocram"))]
compile_error!("Cargo features `dram` and `ocram` are both enabled but only one must be enabled");

// NOTE due to ABI requirements the real entry point, `_start`, is written in
// assembly and lives in the `asm.s` file. That subroutine calls this one.
// NOTE(C ABI) Rust ABI is unspecified / unstable; all calls between Rust code
// and external assembly must use the stable C ABI
#[no_mangle]
unsafe extern "C" fn start() -> ! {
    // NOTE the ROM bootloader can't write the initial values of `.data` to OCRAM because it uses
    // the OCRAM itself. Thus we copy those here, after the ROM bootloader has terminated and
    // there's no risk to corrupt memory
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;
        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;
    }

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    // ensure all the previous writes are committed before any of the following code (which may
    // access `.data`) is executed
    atomic::fence(Ordering::SeqCst);

    /* Initialize some peripherals that will always be configured in this way */
    // enable the RTC with no calibration
    rtc::init();

    // LEDS
    leds::init();
    // turn the white LED on and the blue LED off to indicate we are alive
    leds::set(false, true);

    // the debug accessory (which routes the serial output of the device to the host) is connected
    // to the Armory through a USB-C receptacle. This receptacle is disabled by default so we enable
    // it here. The IC that manages the receptacle (FUSB303) only talks I2C.
    i2c::init();
    if i2c::init_fusb303().is_err() {
        fatal()
    }
    serial::init();

    // on cold boots it seems the receptacle takes quite a while to become active so we have added
    // a delay (~200ms) here to give it time to become ready
    let start = rtc::now();
    while rtc::now() < start + 6554 {
        continue;
    }

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

/// Fatal error during initialization: turn on both LEDs and halt the processor
fn fatal() -> ! {
    leds::set(true, true);

    loop {
        continue;
    }
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
