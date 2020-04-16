use crate::fs;

#[cfg(not(target_arch = "x86_64"))]
use heapless::pool;
pub use heapless::pool::singleton::Pool;
use heapless::pool::Node;

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
