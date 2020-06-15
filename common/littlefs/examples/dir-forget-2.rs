//! [Sanitizer test] Not dropping `ReadDir` should not corrupt memory

// Based on https://github.com/nickray/littlefs2/issues/3 (STR2)

use core::{convert::TryInto, mem};

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

    let filename = b"a.txt\0".try_into().unwrap();

    let mut file = File::create(f, filename).unwrap();
    file.write(b"Hello!").unwrap();
    file.close().unwrap();

    mem::forget(fs::read_dir(f, b".\0".try_into().unwrap()).unwrap());

    fs::remove(f, filename).unwrap();

    println!("OK");
}
