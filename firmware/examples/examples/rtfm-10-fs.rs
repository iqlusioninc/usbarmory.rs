//! Using the FS in a RTFM application
//!
//! Highlights:
//!
//! - The `fs` and `File` API can be used from tasks running at different priorities, without using
//! RTFM's `lock` API -- the `fs` and `File` APIs already use locks internally.
//!
//! - Access control: only tasks that list `Fs` in `resources` can perform FS operations.
//!
//! Expected output:
//!
//! ```
//! (..)
//! [idle] DirEntry { metadata: Metadata { file_name: ".", file_type: Dir, size: 0 } }
//! [foo] created file foo.txt
//! [bar] no FS access
//! [idle] DirEntry { metadata: Metadata { file_name: "..", file_type: Dir, size: 0 } }
//! [idle] DirEntry { metadata: Metadata { file_name: "foo.txt", file_type: File, size: 6 } }
//! ```

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use core::{convert::TryInto, str};

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{self, File, Fs, Path},
    println,
    storage::MbrDevice,
};

fn filename() -> &'static Path {
    b"foo.txt\0".try_into().unwrap()
}

#[rtfm::app]
const APP: () = {
    struct Resources {
        f: Fs,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let emmc = eMMC::take().expect("eMMC").expect("eMMC already taken");

        let mbr = MbrDevice::open(emmc).expect("eMMC not formatted");
        let part = mbr.into_partition(0).expect("eMMC has 0 MBR partitions");

        let format = true;
        let f = Fs::mount(part, format).expect("failed to mount filesystem");

        init::LateResources { f }
    }

    // NOTE `&f` denotes a "share-only" resource; this resource will always appear as a shared
    // reference (`&-`) in tasks. One does not need to call `lock` on these resources to use them.
    #[idle(resources = [&f], spawn = [foo, bar, baz])]
    fn idle(cx: idle::Context) -> ! {
        // resource appears as a shared reference to the resource data: `&Fs`
        let f: &Fs = cx.resources.f;

        for (i, ent) in fs::read_dir(*f, b"/\0".try_into().unwrap())
            .unwrap()
            .into_iter()
            .enumerate()
        {
            let ent = ent.unwrap();

            if i == 1 {
                // these tasks will preempt `idle`
                cx.spawn.foo().unwrap();
                cx.spawn.bar().unwrap();
            }

            println!("[idle] {:?}", ent);
        }

        let filename = filename();
        let file = File::open(*f, filename).unwrap();
        println!("[idle] opened {}", filename);

        // files can be send between tasks
        cx.spawn.baz(file).ok().unwrap();

        usbarmory::reset()
    }

    // this task interrupts `idle`, who's walking over the contents of the `/` directory
    #[task(resources = [&f])]
    fn foo(cx: foo::Context) {
        // makes a copy of the `Fs` handle
        let f: Fs = *cx.resources.f;

        let filename = filename();
        let mut file = File::create(f, filename).unwrap();
        file.write(b"Hello!").unwrap();
        file.close().unwrap();

        println!("[foo] created file {}", filename);
    }

    // this task cannot perform FS operations because it doesn't have access to the `Fs` handle
    // (resource `f`)
    #[task]
    fn bar(_cx: bar::Context) {
        println!("[bar] no FS access");
    }

    #[task]
    fn baz(_cx: baz::Context, mut f: File<Fs>) {
        let filename = filename();
        let mut buf = [0; 32];
        let n = f.read(&mut buf).unwrap();
        println!(
            "[baz] read({}) -> {:?}",
            filename,
            str::from_utf8(&buf[..n])
        );
        f.close().unwrap();
        println!("[baz] closed {}", filename);
    }
};
