//! [Sanitizer test] `lfs_file_t` is properly closed on `File::drop`

// Based on https://github.com/nickray/littlefs2/issues/5

use core::{convert::TryInto, mem};

use littlefs::{filesystem, fs::File, storage};

// RAM `Storage`
storage!(S, block_count = 16);

// Filesystem on top of storage `S`
filesystem!(F, Storage = S, max_open_files = 4, read_dir_depth = 2);

fn main() {
    let s = S::claim().unwrap();
    let f = F::mount(s, true).unwrap();

    foo(f);
    bar(f);

    println!("OK");
}

#[inline(never)]
fn foo(f: F) {
    // `lfs_file_t` will not be closed but its allocation will be leaked (never deallocated)
    // this `lfs_file_t` will remain in the linked list forever
    mem::forget(File::create(f, b"a.txt\0".try_into().unwrap()).unwrap());
}

// linked list operations performed by the `File` API will not corrupt memory
#[inline(never)]
fn bar(f: F) {
    let mut f = File::create(f, b"b.txt\0".try_into().unwrap()).unwrap();
    f.write(b"Hello!").unwrap();
    f.close().unwrap();
}
