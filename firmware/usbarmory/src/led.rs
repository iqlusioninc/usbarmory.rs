//! LEDs

use core::{
    marker::PhantomData,
    sync::atomic::{AtomicU8, Ordering},
};

use pac::gpio::GPIO4;

const BLUE: u32 = 1 << 22;
const WHITE: u32 = 1 << 21;

/// On-board LEDs
///
/// *NOTE* `Leds` is `Send` but its fields (each individual LED) are not
pub struct Leds {
    /// Blue LED
    pub blue: Blue,

    /// White LED
    pub white: White,
}

unsafe impl Send for Leds {}

const NEVER: u8 = 0; // never taken
const TAKEN: u8 = 1; // currently taken
const FREE: u8 = 2; // free to take
static STATE: AtomicU8 = AtomicU8::new(0);

impl Leds {
    /// # Safety
    ///
    /// Creates a singleton from thin air; make sure we never hand out two
    /// instances of it
    unsafe fn new() -> Self {
        Self {
            blue: Blue {
                _not_send_or_sync: PhantomData,
            },
            white: White {
                _not_send_or_sync: PhantomData,
            },
        }
    }

    /// Gets an exclusive handle to the `Leds` singleton
    pub fn take() -> Option<Self> {
        if STATE.load(Ordering::Acquire) == NEVER
            && STATE
                .compare_exchange(NEVER, TAKEN, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        {
            return GPIO4::take().map(|gpio| {
                // GPIO4 was configured in the entry point (`start`)
                drop(gpio); // this seals the configuration

                unsafe { Self::new() }
            });
        }

        if STATE
            .compare_exchange(FREE, TAKEN, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            Some(unsafe { Self::new() })
        } else {
            None
        }
    }

    /// Release the exclusive handle so any other context can take it
    pub fn release(self) {
        STATE.store(FREE, Ordering::Release);
    }
}

/// Blue LED
pub struct Blue {
    _not_send_or_sync: PhantomData<*mut ()>,
}

impl Blue {
    /// Turns the LED off
    pub fn off(&self) {
        // NOTE(borrow_unchecked) the `GPIO4` singleton has been dropped; only
        // the owner of `Leds` can access the peripheral
        GPIO4::borrow_unchecked(|gpio| {
            let old = gpio.DR.read();
            gpio.DR.write(old | BLUE);
        })
    }

    /// Turns the LED on
    pub fn on(&self) {
        // NOTE(borrow_unchecked) the `GPIO4` singleton has been dropped; only
        // the owner of `Leds` can access the peripheral
        GPIO4::borrow_unchecked(|gpio| {
            let old = gpio.DR.read();
            gpio.DR.write(old & !BLUE);
        })
    }

    /// Toggles the LED
    pub fn toggle(&self) {
        // NOTE(borrow_unchecked) the `GPIO4` singleton has been dropped; only
        // the owner of `Leds` can access the peripheral
        GPIO4::borrow_unchecked(|gpio| {
            let old = gpio.DR.read();
            if old & BLUE == 0 {
                gpio.DR.write(old | BLUE)
            } else {
                gpio.DR.write(old & !BLUE)
            }
        })
    }
}

/// White LED
pub struct White {
    _not_send_or_sync: PhantomData<*mut ()>,
}

impl White {
    /// Turns the LED off
    pub fn off(&self) {
        // NOTE(borrow_unchecked) the `GPIO4` singleton has been dropped; only
        // the owner of `Leds` can access the peripheral
        GPIO4::borrow_unchecked(|gpio| {
            let old = gpio.DR.read();
            gpio.DR.write(old | WHITE);
        })
    }

    /// Turns the LED on
    pub fn on(&self) {
        // NOTE(borrow_unchecked) the `GPIO4` singleton has been dropped; only
        // the owner of `Leds` can access the peripheral
        GPIO4::borrow_unchecked(|gpio| {
            let old = gpio.DR.read();
            gpio.DR.write(old & !WHITE);
        })
    }

    /// Toggles the LED
    pub fn toggle(&self) {
        GPIO4::borrow_unchecked(|gpio| {
            let old = gpio.DR.read();
            if old & WHITE == 0 {
                gpio.DR.write(old | WHITE)
            } else {
                gpio.DR.write(old & !WHITE)
            }
        })
    }
}
