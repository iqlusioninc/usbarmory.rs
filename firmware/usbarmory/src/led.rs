//! LEDs

use rac::gpio;

// TODO turn these into owned singletons

const BLUE: u32 = 1 << 22;
const WHITE: u32 = 1 << 21;

/// Blue LED
pub struct Blue;

impl Blue {
    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old | BLUE);
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old & !BLUE);
        }
    }
}

/// White LED
pub struct White;

impl White {
    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old | WHITE);
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old & !WHITE);
        }
    }
}
