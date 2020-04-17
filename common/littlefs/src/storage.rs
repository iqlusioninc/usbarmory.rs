//! Storage interface

use crate::{consts, io};

/// Interface to an storage device
///
/// # Safety
///
/// Implementer (`Self`) must be an owned singleton handle: i.e. only a single instance of `Self`
/// can ever exist
pub unsafe trait Storage {
    /// Number of blocks associated to this storage device
    const BLOCK_COUNT: u32;

    /// Reads data from the storage device
    // NOTE(`Result<()`) C API expects the return value to be `<= 0`
    fn read(&self, off: usize, buf: &mut [u8]) -> io::Result<()>;

    /// Write data to the storage device
    // NOTE(`Result<()`) C API expects the return value to be `<= 0`
    fn write(&self, off: usize, data: &[u8]) -> io::Result<()>;

    /// Erases data from the storage device
    // NOTE(`Result<()`) C API expects the return value to be `<= 0`
    fn erase(&self, off: usize, len: usize) -> io::Result<()>;

    /// Locks the storage device; any attempt to write to it will result in a panic
    fn lock(&self);

    /// Unlocks the storage device, re-allowing writes to it
    fn unlock(&self);
}

/// Declares a storage device backed by a statically block of RAM
#[macro_export]
macro_rules! storage {
    ($storage:ident, block_count=$n:expr) => {
        pub struct $storage {
            _inner: $crate::NotSendOrSync,
        }

        impl $storage {
            pub fn claim() -> Option<$storage> {
                use core::sync::atomic::{AtomicBool, Ordering};

                use $crate::NotSendOrSync;

                static ONCE: AtomicBool = AtomicBool::new(false);

                if ONCE
                    .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    Some(Self {
                        _inner: unsafe { NotSendOrSync::new() },
                    })
                } else {
                    None
                }
            }

            /// # Safety
            /// Aliases memory; only one instance of `Inner` must be alive at any point in time
            unsafe fn inner() -> $crate::storage::Inner {
                use $crate::storage::Inner;

                static mut MEMORY: [u8; $n * $crate::consts::BLOCK_SIZE as usize] =
                    [0; $n * $crate::consts::BLOCK_SIZE as usize];

                Inner::new(&mut MEMORY)
            }
        }

        unsafe impl $crate::storage::Storage for $storage {
            const BLOCK_COUNT: u32 = $n;

            fn read(&self, offset: usize, buf: &mut [u8]) -> $crate::io::Result<usize> {
                unsafe { Self::inner().read(offset, buf) }
            }

            fn write(&self, offset: usize, data: &[u8]) -> $crate::io::Result<usize> {
                unsafe { Self::inner().write(offset, data) }
            }

            fn erase(&self, offset: usize, len: usize) -> $crate::io::Result<usize> {
                unsafe { Self::inner().erase(offset, len) }
            }
        }
    };
}

#[doc(hidden)]
pub struct Inner {
    data: &'static mut [u8],
}

#[doc(hidden)]
impl Inner {
    pub fn new(data: &'static mut [u8]) -> Self {
        Self { data }
    }

    pub fn read(&self, offset: usize, buf: &mut [u8]) -> io::Result<usize> {
        let read_size = consts::READ_SIZE as usize;

        debug_assert!(offset % read_size == 0);
        debug_assert!(buf.len() % read_size == 0);

        let n = buf.len();
        buf.copy_from_slice(&self.data[offset..offset + n]);
        Ok(n)
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> io::Result<usize> {
        let write_size = consts::WRITE_SIZE as usize;

        debug_assert!(offset % write_size == 0);
        debug_assert!(data.len() % write_size == 0);

        let n = data.len();
        self.data[offset..offset + n].copy_from_slice(data);
        Ok(n)
    }

    pub fn erase(&mut self, offset: usize, len: usize) -> io::Result<usize> {
        const ERASE_VALUE: u8 = 0xFF;

        let block_size = consts::BLOCK_SIZE as usize;

        debug_assert!(offset % block_size == 0);
        debug_assert!(len % block_size == 0);
        for byte in self.data[offset..offset + len].iter_mut() {
            *byte = ERASE_VALUE;
        }
        Ok(len)
    }
}
