//! Input / Output

/// Result with Error variant set to I/O error
pub type Result<T> = core::result::Result<T, Error>;

/// Definition of errors that might be returned by filesystem functionality.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    /// Input / output error occurred.
    Io,
    /// File or filesystem was corrupt.
    Corruption,
    /// No entry found with that name.
    NoSuchEntry,
    /// File or directory already exists.
    EntryAlreadyExisted,
    /// Path name is not a directory.
    PathNotDir,
    /// Path specification is to a directory.
    PathIsDir,
    /// Directory was not empty.
    DirNotEmpty,
    /// Bad file descriptor.
    BadFileDescriptor,
    /// File is too big.
    FileTooBig,
    /// Incorrect value specified to function.
    Invalid,
    /// No space left available for operation.
    NoSpace,
    /// No memory available for completing request.
    NoMemory,
    /// No attribute or data available
    NoAttribute,
    /// Filename too long
    FilenameTooLong,
    /// Unknown error occurred, integer code specified.
    Unknown(i32),
}

pub(crate) fn check_ret(ret: ll::lfs_error) -> Result<u32> {
    match ret {
        n if n >= 0 => Ok(n as u32),
        // negative codes
        ll::lfs_error_LFS_ERR_IO => Err(Error::Io),
        ll::lfs_error_LFS_ERR_CORRUPT => Err(Error::Corruption),
        ll::lfs_error_LFS_ERR_NOENT => Err(Error::NoSuchEntry),
        ll::lfs_error_LFS_ERR_EXIST => Err(Error::EntryAlreadyExisted),
        ll::lfs_error_LFS_ERR_NOTDIR => Err(Error::PathNotDir),
        ll::lfs_error_LFS_ERR_ISDIR => Err(Error::PathIsDir),
        ll::lfs_error_LFS_ERR_NOTEMPTY => Err(Error::DirNotEmpty),
        ll::lfs_error_LFS_ERR_BADF => Err(Error::BadFileDescriptor),
        ll::lfs_error_LFS_ERR_FBIG => Err(Error::FileTooBig),
        ll::lfs_error_LFS_ERR_INVAL => Err(Error::Invalid),
        ll::lfs_error_LFS_ERR_NOSPC => Err(Error::NoSpace),
        ll::lfs_error_LFS_ERR_NOMEM => Err(Error::NoMemory),
        ll::lfs_error_LFS_ERR_NOATTR => Err(Error::NoAttribute),
        ll::lfs_error_LFS_ERR_NAMETOOLONG => Err(Error::FilenameTooLong),
        _ => Err(Error::Unknown(ret)),
    }
}
