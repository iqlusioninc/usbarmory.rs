// **WARNING** changing these values can cause undefined behavior

// block must be multiple of cache
// cache must be multiple of read
// cache must be multiple of write
pub const BLOCK_SIZE: u32 = CACHE_SIZE;
pub const CACHE_SIZE: u32 = READ_SIZE;
pub const READ_SIZE: u32 = WRITE_SIZE;
pub const WRITE_SIZE: u32 = 512;

pub const ATTRBYTES_MAX: u32 = 1022;
pub const FILEBYTES_MAX: u32 = 2_147_483_647;
pub const LOOKAHEADWORDS_SIZE: u32 = 16;
pub const BLOCK_CYCLES: i32 = -1;

// NOTE it seems these HAVE to be 256 because of that's the size of the `lfs_info.name`
pub const FILENAME_MAX_PLUS_ONE: u32 = 255 + 1;
pub const PATH_MAX_PLUS_ONE: usize = 255 + 1;
