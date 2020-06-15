use core::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
};

use generic_array::{ArrayLength, GenericArray};
#[cfg(not(target_arch = "x86_64"))]
use heapless::pool;
pub use heapless::pool::singleton::Pool;
use heapless::pool::Node;

use crate::{fs, io};

#[cfg(all(target_arch = "x86_64", feature = "unsafe-x86"))]
macro_rules! pool {
    ($(#[$($attr:tt)*])* $ident:ident: $ty:ty) => {
        /// A global handle to the memory pool
        pub struct $ident;

        impl Pool for $ident {
            type Data = $ty;

            fn ptr() -> &'static heapless::pool::Pool<$ty> {
                $(#[$($attr)*])*
                static mut $ident: heapless::pool::Pool<$ty> = heapless::pool::Pool::new();

                unsafe { &$ident }
            }
        }
    };
}

pool!(D: ll::lfs_dir);

pub type DNode = Node<ll::lfs_dir>;

pool!(F: fs::FileState);

pub type FNode = Node<fs::FileState>;

// a heapless Arena
// NOTE this is missing a `Drop` implementation but it doesn't matter because we'll `close` all
// files by hand (rather than relying on destructors running) in `Fs.sync`
pub(crate) struct Arena<T, N>
where
    N: ArrayLength<T>,
{
    buffer: UnsafeCell<MaybeUninit<GenericArray<T, N>>>,
    pos: Cell<usize>,
}

impl<T, N> Arena<T, N>
where
    N: ArrayLength<T>,
{
    pub(crate) fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(MaybeUninit::uninit()),
            pos: Cell::new(0),
        }
    }

    pub(crate) fn alloc(&self, value: T) -> io::Result<&mut T> {
        let i = self.pos.get();
        if i < N::USIZE {
            let bufferp = self.buffer.get() as *mut T;
            unsafe { bufferp.add(i).write(value) }
            self.pos.set(i + 1);
            Ok(unsafe { &mut *bufferp.add(i) })
        } else {
            Err(io::Error::NoMemory)
        }
    }

    pub(crate) fn has_space(&self) -> bool {
        self.pos.get() < N::USIZE
    }

    pub(crate) fn into_iter(self) -> IntoIter<T, N> {
        IntoIter {
            buffer: self.buffer,
            len: self.pos.get(),
            pos: 0,
        }
    }
}

pub(crate) struct IntoIter<T, N>
where
    N: ArrayLength<T>,
{
    buffer: UnsafeCell<MaybeUninit<GenericArray<T, N>>>,
    len: usize,
    pos: usize,
}

impl<T, N> Iterator for IntoIter<T, N>
where
    N: ArrayLength<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.pos < self.len {
            let item = unsafe { self.buffer.get().cast::<T>().add(self.pos).read() };
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}
