//! Using the FS in a RTFM application
//!
//! Highlights:
//!
//! - The `fs` and `File` API can be used from tasks running at different priorities, without using
//! RTFM's `lock` API -- the `fs` and `File` APIs already use locks internally.
//!
//! - Access control: only tasks that list `Fs` in `resources` can perform FS operations.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use core::convert::TryInto;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    fs::{self, File, Fs},
    println,
    storage::MbrDevice,
};

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

    #[idle(resources = [&f], spawn = [foo, bar])]
    fn idle(cx: idle::Context) -> ! {
        let f = *cx.resources.f;

        let mut first = true;
        for ent in fs::read_dir(f, b"/\0".try_into().unwrap()).unwrap() {
            let ent = ent.unwrap();

            if first {
                first = false;
                // these tasks will preempt `idle`
                cx.spawn.foo().unwrap();
                cx.spawn.bar().unwrap();
            }

            println!("[idle] {:?}", ent);
        }

        usbarmory::reset()
    }

    // interrupts `idle`, who's walking over the contents of the `/` directory
    #[task(resources = [&f])]
    fn foo(cx: foo::Context) {
        let f = *cx.resources.f;

        let mut file = File::create(f, b"foo.txt\0".try_into().unwrap()).unwrap();
        file.write(b"Hello!").unwrap();
        file.close().unwrap();

        println!("[foo] created file foo.txt");
    }

    // this task cannot perform FS operations because it doesn't have access to the `Fs` handle
    // (resource `f`)
    #[task]
    fn bar(_cx: bar::Context) {
        println!("[bar]");
    }
};
