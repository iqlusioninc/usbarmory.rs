//! RTFM hardware tasks
//!
//! An "echo" application on top of the serial interface. Data received over the
//! serial interface will be echoed back over the serial interface. If you type
//! "Hello"; you'll see "Hello" on the serial console. On each byte of data
//! received the state of the blue LED is toggled. Pressing Enter in the serial
//! interface will reboot the device and bring back the u-boot console.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    led::Blue,
    serial::{Event, Serial},
};

#[rtfm::app]
const APP: () = {
    struct Resources {
        led: Blue,
        serial: Serial,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let led = Blue::take().expect("UNREACHABLE");
        let serial = Serial::take().expect("UNREACHABLE");

        // receiving data over the serial interface triggers the UART2 interrupt
        // signal
        serial.listen(Event::ReceiveReady);

        init::LateResources { led, serial }
    }

    // as no `idle` function was defined the device will go to light sleep
    // (saves some power) when there's no task to service

    // This is a hardware task bound to the UART2 interrupt signal
    //
    // This task will be started when a new byte of data is received over the
    // serial interface. Hardware tasks cannot be `spawn`-ed.
    #[task(binds = UART2, resources = [led, serial])]
    fn on_new_data(cx: on_new_data::Context) {
        let serial = cx.resources.serial;
        let led = cx.resources.led;

        if let Some(byte) = serial.read() {
            // reboot the device when the Enter key is received
            if byte == b'\r' {
                usbarmory::reset();
            }

            led.toggle();
            serial.write(byte);
        } else {
            // this could be reached under these scenarios
            // - the serial interface is also listening for events that have
            //   nothing to do with receiving data (e.g. transmission complete)
            // - the UART2 interrupt signal was artificially generated using the
            //   GIC (there's no API for this ATM)
            #[cfg(debug_assertions)]
            unreachable!()
        }
    }
};
