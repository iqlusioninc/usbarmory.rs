//! Temporal quantification

use core::{ops, time::Duration};

use pac::snvs_hp::SNVS_HP;

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only
/// with Duration.
#[derive(Clone, Copy)]
pub struct Instant {
    value: u64,
}

impl Instant {
    /// Returns an instant corresponding to "now".
    pub fn now() -> Self {
        // NOTE(borrow_unchecked) `SNVS_HP` has been dropped at this point; this
        // is the only method that will access these registers
        SNVS_HP::borrow_unchecked(|snvs| {
            // The ULRM recommends that we perform two consecutive reads of
            // these registers because the RTC Clock is not synchronized with
            // the processor clock so we could observe torn reads. The ULRM says
            // that at most this can result in three reads of these pair of
            // registers
            let mut high = snvs.RTCMR.read();
            let mut low = snvs.RTCLR.read();

            loop {
                let new_high = snvs.RTCMR.read();
                let new_low = snvs.RTCLR.read();

                if new_low == low && new_high == high {
                    return Instant {
                        value: u64::from(high) << 32 | u64::from(low),
                    };
                } else {
                    // potential torn read; try again
                    low = new_low;
                    high = new_high;
                }
            }
        })
    }

    /// Returns the amount of time elapsed from another instant to this one.
    pub fn duration_since(self, earlier: Instant) -> Duration {
        assert!(
            self.value <= earlier.value,
            "supplied instant is later than self"
        );

        // The RTC is clocked at `32_768` Hz
        const RTC_FREQUENCY: u64 = 1 << 15;

        let ticks = self.value - earlier.value;
        Duration::new(
            // this should lower to a right shift
            ticks / RTC_FREQUENCY,
            // 1 tick is equal to `1e9 / RTC_FREQUENCY` nanos; the fraction
            // can be reduced to `1_953_125 / (1 << 9)`
            (1_953_125 * (ticks % RTC_FREQUENCY) / (1 << 9)) as u32,
        )
    }

    /// Returns the amount of time elapsed since this instant was created.
    pub fn elapsed(self) -> Duration {
        Instant::now() - self
    }
}

impl ops::Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}
