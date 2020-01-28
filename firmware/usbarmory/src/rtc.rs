//! Real Time Counter

use core::{marker::PhantomData, time::Duration};

use pac::snvs::SNVS;

/// A handle to the initialized RTC
///
/// This handle is `Copy` and `Send`; once initialized the RTC can be read from
/// any context -- the handle can be sent to any context
#[derive(Clone, Copy)]
pub struct Rtc {
    _not_sync: PhantomData<*mut ()>,
}

unsafe impl Send for Rtc {}

impl Rtc {
    /// Initializes the RTC
    ///
    /// The `RTC` can only be initialized once. This function will return `Some`
    /// at most once. This function will return `None` if the `SNVS` peripheral
    /// singleton has already been `take`-n.
    pub fn initialize() -> Option<Self> {
        /// HP Real-Time Counter Enable
        const SNVS_HPCR_RTC_EN: u32 = 1 << 0;

        // FIXME we don't want to take the whole SNVS peripheral here; just some
        // registers
        SNVS::take().map(|snvs| {
            // enable the RTC with no calibration
            snvs.HPCR.write(SNVS_HPCR_RTC_EN);

            // seal the SNVS configuration
            drop(snvs);

            Rtc {
                _not_sync: PhantomData,
            }
        })
    }

    /// Returns the time elapsed since the RTC was initialized
    ///
    /// The RTC is a monotonic timer that can't be reset.
    pub fn elapsed(&self) -> Duration {
        // NOTE(borrow_unchecked) `SNVS` has been dropped at this point; this is
        // the only method that will access these registers
        SNVS::borrow_unchecked(|snvs| {
            // The RM recommends that we perform two consecutive reads of these
            // registers because the RTC Clock is not synchronized with the
            // processor clock so we could observe torn reads. The RM says that
            // at most this can result in three reads of these pair of registers
            let mut high = snvs.HPRTCMR.read();
            let mut low = snvs.HPRTCLR.read();

            loop {
                let new_high = snvs.HPRTCMR.read();
                let new_low = snvs.HPRTCLR.read();

                if new_low == low && new_high == high {
                    // The RTC is clocked at `32_768` Hz
                    const RTC_FREQUENCY: u64 = 1 << 15;

                    let ticks = u64::from(high) << 32 | u64::from(low);
                    return Duration::new(
                        // this should lower to a right shift
                        ticks / RTC_FREQUENCY,
                        // 1 tick = `1e9 / RTC_FREQUENCY` nanos; the fraction
                        // can be reduced to `1_953_125 / (1 << 9)`
                        (1_953_125 * (ticks % RTC_FREQUENCY) / (1 << 9)) as u32,
                    );
                } else {
                    // potential torn read; try again
                    low = new_low;
                    high = new_high;
                }
            }
        })
    }
}
