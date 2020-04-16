//! [Sanitizer test] `lfs_dir_t` is properly closed on `ReadDir::drop`

// Based on https://github.com/nickray/littlefs2/issues/3 (STR1)

use core::convert::TryInto;

use littlefs::{
    filesystem,
    fs::{self, File},
    storage,
};

// RAM `Storage`
storage!(S, block_count = 16);

// Filesystem on top of storage `S`
filesystem!(F, Storage = S, max_open_files = 4, read_dir_depth = 2);

fn main() {
    let s = S::claim().unwrap();
    let f = F::mount(s, true).unwrap();

    foo(f);
    bar(f);
}

#[inline(never)]
fn foo(f: F) {
    // `ReaDir` will close `lfs_dir_t` on drop
    drop(fs::read_dir(f, b".\0".try_into().unwrap()).unwrap());
}

// linked list operations performed by the `File` API will not corrupt memory
#[inline(never)]
fn bar(f: F) {
    let mut file = File::create(f, b"a.txt\0".try_into().unwrap()).unwrap();
    file.write(b"Hello!").unwrap();
    file.close().unwrap();
}
