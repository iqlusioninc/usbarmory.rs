use core::{fmt, ops, ptr::NonNull, time::Duration};

use usb_device::{bus::PollResult, endpoint::EndpointAddress, UsbDirection};

use crate::time::Instant;

/// Aligns the inner value to a 2KB boundary
#[repr(align(2048))]
pub struct Align2K<T> {
    pub inner: T,
}

/// Aligns the inner value to a 4KB boundary
#[repr(align(4096))]
pub struct Align4K<T> {
    pub inner: T,
}

/// Always contains the value zero
#[repr(transparent)]
pub struct Reserved {
    inner: u32,
}

impl Reserved {
    pub const fn new() -> Self {
        Reserved { inner: 0 }
    }
}

/// Like `&'_ T` but with move semantics and constrained access
///
/// We want move semantics to be able to kill the reference by `drop`-ing the
/// value.
pub struct Ref<T> {
    inner: NonNull<T>,
}

impl<T> Ref<T> {
    /// # Safety
    /// Lifetime is lost. Lifetime must be tracked by the caller
    pub unsafe fn new(p: &T) -> Self {
        Self {
            inner: NonNull::from(p),
        }
    }

    /// # Safety
    /// Lifetime must be tracked by the caller
    pub unsafe fn new_unchecked(p: *const T) -> Self {
        Self {
            inner: NonNull::new_unchecked(p as *mut T),
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }
}

impl<T> ops::Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.as_ref() }
    }
}

/// Waits until `cond` returns true
///
/// Returns `Err` if the operation timed out
pub fn wait(mut cond: impl FnMut() -> bool, timeout: Duration) -> Result<(), ()> {
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

pub fn epaddr2dqhidx(ep_addr: EndpointAddress) -> usize {
    2 * ep_addr.index() + if ep_addr.is_out() { 0 } else { 1 }
}

pub fn dqhidx2epaddr(idx: usize) -> EndpointAddress {
    let dir = if idx % 2 == 0 {
        UsbDirection::Out
    } else {
        UsbDirection::In
    };
    EndpointAddress::from_parts(idx / 2, dir)
}

pub fn epaddr2endptmask(ep_addr: EndpointAddress) -> u32 {
    if ep_addr.is_out() {
        1 << ep_addr.index()
    } else {
        1 << (16 + ep_addr.index())
    }
}

#[derive(Clone, Copy)]
pub struct Data {
    pub ep_in_complete: u16,
    pub ep_setup: u16,
    pub ep_out: u16,
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Data");
        if self.ep_in_complete != 0 {
            s.field("ep_in_complete", &Hex(self.ep_in_complete));
        }
        if self.ep_setup != 0 {
            s.field("ep_setup", &Hex(self.ep_setup));
        }
        if self.ep_out != 0 {
            s.field("ep_out", &Hex(self.ep_out));
        }
        s.finish()
    }
}

impl From<Data> for PollResult {
    fn from(data: Data) -> PollResult {
        PollResult::Data {
            ep_in_complete: data.ep_in_complete,
            ep_out: data.ep_out,
            ep_setup: data.ep_setup,
        }
    }
}

pub struct Hex<T>(pub T);

impl fmt::Debug for Hex<u16> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 <= 0xff {
            write!(f, "{:#04x}", self.0)
        } else {
            write!(f, "{:#06x}", self.0)
        }
    }
}

impl fmt::Debug for Hex<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 <= 0xff {
            write!(f, "{:#04x}", self.0)
        } else if self.0 <= 0xffff {
            write!(f, "{:#06x}", self.0)
        } else {
            write!(f, "{:#06x}_{:04x}", self.0 >> 16, self.0 & 0xffff)
        }
    }
}

/// Iterates over the indices of the bits of a word that are set to 1
impl OneIndices {
    pub fn of(word: u32) -> Self {
        Self { inner: word }
    }
}

pub struct OneIndices {
    inner: u32,
}

impl Iterator for OneIndices {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.inner == 0 {
            None
        } else {
            let i = self.inner.trailing_zeros();
            self.inner &= !(1 << i);
            Some(i as u8)
        }
    }
}
