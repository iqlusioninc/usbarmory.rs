#![no_std]

use core::time::Duration;

// Some random VID & PID values for testing purposes
/// `commit_year as VID`
pub const VID: u16 = 0x2020; // year

/// `commit_month_day as PID`
pub const PID: u16 = 0x0212; // month-day

/// The time between two start of frames (SoF)
pub fn frame() -> Duration {
    Duration::from_millis(1)
}

/// The time between two start of micro-frames
pub fn microframe() -> Duration {
    Duration::from_micros(125)
}
