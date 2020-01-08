All code under this directory is to be compiled for the embedded target: ARM
Cortex-A7.

There are two compilation targets (see `--target` in `rustc -h`) for this
embedded target:

- armv7-none-eabihf, hard float ABI; float operations are performed on the
  hardware FPU. NOTE: currently untested

- armv7-none-eabi, soft float ABI; float operations are emulated

The default compilation target for the whole directory is currently set to the
most compatible soft float target. If you need to use the hard float target
modify `.cargo/config`, or pass `--target armv7-none-eabihf` to Xargo.

*NOTE* all the code under this directory needs Rust nightly and Xargo. See the
README in the [`usbarmory`](usbarmory) directory for more information.
