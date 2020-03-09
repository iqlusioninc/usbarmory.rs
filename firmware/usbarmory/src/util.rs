use core::time::Duration;

use crate::time::Instant;

/// Waits until `cond` returns true
///
/// Returns `Err` if the operation timed out
pub fn wait_for_or_timeout(mut cond: impl FnMut() -> bool, timeout: Duration) -> Result<(), ()> {
    let start = Instant::now();
    while !cond() {
        // instead of busy waiting flush the in-memory logger
        crate::memlog_try_flush();

        if start.elapsed() > timeout {
            return Err(());
        }
    }
    Ok(())
}

#[repr(align(4))]
pub struct Align4<T> {
    pub inner: T,
}

pub fn round_down(n: usize, m: usize) -> usize {
    n - (n % m)
}
