//! A very opinionated littlefs (v2.1.4) wrapper
//!
//! **NOTE** This contains bits and pieces of the `littlefs2` crate
//!
//! # Limitations
//!
//! - All the filesystem settings are hard-coded in this crate (see `src/consts.rs`)
//!
//! - Buffers (used by the `littlefs` C library) are managed in memory pools; they cannot be
//! allocated on the stack (atm). Furthermore, these managed buffers will be shared by *all* mounted
//! filesystems so it's easy to run out of memory if you mount many filesystems.
//!
//! TL;DR this library is meant to be used in programs that will only use a *single* filesystem
//!
//! # Example usage
//!
//! ``` ignore
//! use littlefs::{
//!     filesystem,
//!     fs::{self, File},
//!     storage,
//! };
//!
//! // RAM `Storage`
//! storage!(S, block_count = 16);
//!
//! // Filesystem on top of storage `S`
//! filesystem!(F, Storage = S, max_open_files = 4, read_dir_depth = 2);
//!
//! // claim ownership over the ram storage
//! let storage = S::claim().expect("Storage already claimed");
//!
//! // mount the filesystem but format a the storage device first
//! let format = true;
//! let f = F::mount(storage, format).unwrap();
//!
//! // create a directory
//! fs::create_dir(f, b"/foo\0".try_into().unwrap()).unwrap();
//!
//! // create a file
//! let mut f1 = File::create(f, b"/foo/bar.txt\0".try_into().unwrap()).unwrap();
//!
//! // write data to the file cache
//! f1.write(b"Hello, world!").unwrap();
//!
//! // commit data to the storage device and discard the file handle
//! f1.close().unwrap();
//!
//! // iterate over the contents of the root directory
//! for entry in fs::read_dir(f, b"/\0".try_into().unwrap()).unwrap() {
//!     println!("{:?}", entry.unwrap());
//! }
//! ```
//!
//! # Cargo features
//!
//! The `unsafe-x86` feature is required to use this crate on the x86_64 architecture. It is
//! *unsafe* to enable the Cargo feature because `heapless::Pool` is not (yet) thread-safe on
//! x86_64. If you enable that feature note that manually calling `D::alloc` or `FS::alloc` can
//! result in memory corruption.
//!
//! The `fs` and `File` API are sound to use on x86_64 provided that all filesystems are used from
//! the same thread -- the `Filesystem` trait will prevent you (at compile time) from using *one*
//! filesystem from different threads but won't prevent you from mounting different filesystems from
//! different threads. All these issues can be avoided by using a *single* filesystem in the
//! application, which is the intended use case.

#![no_std]
#![warn(rust_2018_idioms, unused_qualifications)]

use core::marker::PhantomData;

#[cfg(any(not(target_arch = "x86_64"), feature = "unsafe-x86"))]
#[doc(hidden)]
pub mod consts;
#[cfg(any(not(target_arch = "x86_64"), feature = "unsafe-x86"))]
pub mod fs;
#[cfg(any(not(target_arch = "x86_64"), feature = "unsafe-x86"))]
pub mod io;
#[cfg(any(not(target_arch = "x86_64"), feature = "unsafe-x86"))]
#[doc(hidden)]
pub mod mem;
#[cfg(any(not(target_arch = "x86_64"), feature = "unsafe-x86"))]
pub mod path;
#[cfg(any(not(target_arch = "x86_64"), feature = "unsafe-x86"))]
pub mod storage;

#[cfg(all(target_arch = "x86_64", not(feature = "unsafe-x86")))]
compile_error!(
    "the `unsafe-x86` Cargo feature must be enabled -- READ THE DOCS FIRST -- to use this crate on x86_64"
);

/// Implementation detail
///
/// We use this type to *prevent* the creation of singletons in safe code -- in particular we do
/// *not* want the `Filesystem` singleton (handle) to be created before the filesystem has been
/// mounted
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Private {
    _inner: PhantomData<*mut ()>,
}

#[doc(hidden)]
impl Private {
    /// Macro implementation detail
    ///
    /// # Safety
    /// `unsafe` to prevent construction of singletons in safe code
    pub unsafe fn new() -> Self {
        Self {
            _inner: PhantomData,
        }
    }
}

#[cfg(feature = "sync-cortex-a")]
unsafe impl Send for Private {}

#[cfg(feature = "sync-cortex-a")]
unsafe impl Sync for Private {}

// not interrupt/thread safe
#[cfg(not(feature = "sync-cortex-a"))]
pub fn lock<T>(f: impl FnOnce() -> T) -> T {
    f()
}

// interrupt safe
#[cfg(feature = "sync-cortex-a")]
pub fn lock<T>(f: impl FnOnce() -> T) -> T {
    cortex_a::no_interrupts(f)
}

/// Implementation detail
/// Variation of `Private` that's always `!Send` and `!Sync`
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct NotSendOrSync {
    _inner: PhantomData<*mut ()>,
}

#[doc(hidden)]
impl NotSendOrSync {
    /// Macro implementation detail
    ///
    /// # Safety
    /// `unsafe` to prevent construction of singletons in safe code
    pub unsafe fn new() -> Self {
        Self {
            _inner: PhantomData,
        }
    }
}
