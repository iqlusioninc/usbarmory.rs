//! Paths

use core::{convert::TryFrom, fmt, mem::MaybeUninit, ops, ptr, slice, str};

use cstr_core::CStr;
use cty::c_char;

use crate::consts;

/// A path
///
/// Paths must be null terminated ASCII strings
pub struct Path {
    inner: CStr,
}

impl Path {
    /// Creates a path from a byte buffer
    ///
    /// The buffer will be first interpreted as a `CStr` and then checked to be comprised only of
    /// ASCII characters.
    pub fn from_bytes_with_nul<'b>(bytes: &'b [u8]) -> Result<&'b Self> {
        let cstr = CStr::from_bytes_with_nul(bytes).map_err(|_| Error::NotCStr)?;
        Self::from_cstr(cstr)
    }

    /// Unchecked version of `from_bytes_with_nul`
    pub unsafe fn from_bytes_with_nul_unchecked<'b>(bytes: &'b [u8]) -> &'b Self {
        &*(bytes as *const [u8] as *const Path)
    }

    /// Creates a path from a C string
    ///
    /// The string will be checked to be comprised only of ASCII characters
    // XXX should we reject empty paths (`""`) here?
    pub fn from_cstr<'s>(cstr: &'s CStr) -> Result<&'s Self> {
        let bytes = cstr.to_bytes();
        let n = cstr.to_bytes().len();
        if n + 1 > consts::PATH_MAX_PLUS_ONE {
            Err(Error::TooLarge)
        } else if bytes.is_ascii() {
            Ok(unsafe { Self::from_cstr_unchecked(cstr) })
        } else {
            Err(Error::NotAscii)
        }
    }

    /// Unchecked version of `from_cstr`
    pub unsafe fn from_cstr_unchecked<'s>(cstr: &'s CStr) -> &'s Self {
        &*(cstr as *const CStr as *const Path)
    }

    /// Returns the inner pointer to this C string.
    pub(crate) fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr()
    }

    /// Creates an owned `PathBuf` with `path` adjoined to `self`.
    pub fn join(&self, path: &Path) -> PathBuf {
        let mut p = PathBuf::from(self);
        p.push(path);
        p
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        // NOTE(unsafe) ASCII is valid UTF-8
        unsafe { str::from_utf8_unchecked(self.inner.to_bytes()) }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'b> TryFrom<&'b [u8]> for &'b Path {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<&Path> {
        Path::from_bytes_with_nul(bytes)
    }
}

// without this you need to slice byte string literals (`b"foo\0"[..].try_into()`)
macro_rules! array_impls {
    ($($N:expr),+) => {
        $(
            impl<'b> TryFrom<&'b [u8; $N]> for &'b Path {
                type Error = Error;

                fn try_from(bytes: &[u8; $N]) -> Result<&Path> {
                    Path::from_bytes_with_nul(&bytes[..])
                }
            }
        )+
    }
}

array_impls!(
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32
);

/// An owned, mutable path
#[derive(Clone)]
pub struct PathBuf {
    buf: [c_char; consts::PATH_MAX_PLUS_ONE],
    // NOTE `len` DOES include the final null byte
    len: usize,
}

impl PathBuf {
    pub(crate) unsafe fn from_buffer(buf: [c_char; consts::PATH_MAX_PLUS_ONE]) -> Self {
        let len = c_stubs::strlen(buf.as_ptr()) + 1 /* null byte */;
        PathBuf { buf, len }
    }

    /// Extends `self` with `path`
    pub fn push(&mut self, path: &Path) {
        match path.as_ref() {
            // no-operation
            "" => return,

            // `self` becomes `/` (root), to match `std::Path` implementation
            "/" => {
                self.buf[0] = b'/' as c_char;
                self.buf[1] = 0;
                self.len = 2;
                return;
            }
            _ => {}
        }

        let src = path.as_ref().as_bytes();
        let needs_separator = self
            .as_ref()
            .as_bytes()
            .last()
            .map(|byte| *byte != b'/')
            .unwrap_or(false);
        let slen = src.len();
        assert!(
            self.len
                + slen
                + if needs_separator {
                    // b'/'
                    1
                } else {
                    0
                }
                <= consts::PATH_MAX_PLUS_ONE
        );

        let len = self.len;
        unsafe {
            let mut p = self.buf.as_mut_ptr().cast::<u8>().add(len - 1);
            if needs_separator {
                p.write(b'/');
                p = p.add(1);
                self.len += 1;
            }
            ptr::copy_nonoverlapping(src.as_ptr(), p, slen);
            p.add(slen).write(0); // null byte
            self.len += slen;
        }
    }
}

impl From<&Path> for PathBuf {
    fn from(path: &Path) -> Self {
        let mut buf = MaybeUninit::<[c_char; consts::PATH_MAX_PLUS_ONE]>::uninit();
        let bytes = path.as_ref().as_bytes();
        let len = bytes.len();
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr().cast(), len) }
        Self {
            buf: unsafe { buf.assume_init() },
            len: len + 1,
        }
    }
}

impl ops::Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Path {
        unsafe {
            Path::from_bytes_with_nul_unchecked(slice::from_raw_parts(
                self.buf.as_ptr().cast(),
                self.len,
            ))
        }
    }
}

impl fmt::Debug for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Path as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Path as fmt::Display>::fmt(self, f)
    }
}

/// Errors that arise from converting byte buffers into paths
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// Byte buffer contains non-ASCII characters
    NotAscii,
    /// Byte buffer is not a C string
    NotCStr,
    /// Byte buffer is too long (longer than `consts::PATH_MAX_PLUS_ONE`)
    TooLarge,
}

/// Result type that has its Error variant set to `path::Error`
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(tests)]
mod tests {
    use super::Path;

    #[test]
    fn join() {
        let empty = Path::from_bytes_with_nul(b"\0").unwrap();
        let slash = Path::from_bytes_with_nul(b"/\0").unwrap();
        let a = Path::from_bytes_with_nul(b"a\0").unwrap();
        let b = Path::from_bytes_with_nul(b"b\0").unwrap();

        assert_eq!(empty.join(empty).as_ref(), "");
        assert_eq!(empty.join(slash).as_ref(), "/");
        assert_eq!(empty.join(a).as_ref(), "a");
        assert_eq!(empty.join(b).as_ref(), "b");

        assert_eq!(slash.join(empty).as_ref(), "/");
        assert_eq!(slash.join(slash).as_ref(), "/");
        assert_eq!(slash.join(a).as_ref(), "/a");
        assert_eq!(slash.join(b).as_ref(), "/b");

        assert_eq!(a.join(empty).as_ref(), "a");
        assert_eq!(a.join(slash).as_ref(), "/");
        assert_eq!(a.join(a).as_ref(), "a/a");
        assert_eq!(a.join(b).as_ref(), "a/b");

        assert_eq!(b.join(empty).as_ref(), "b");
        assert_eq!(b.join(slash).as_ref(), "/");
        assert_eq!(b.join(a).as_ref(), "b/a");
        assert_eq!(b.join(b).as_ref(), "b/b");
    }
}
