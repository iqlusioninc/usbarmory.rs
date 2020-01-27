All code under this directory is to be compiled for the embedded target: ARM
Cortex-A7.

There are two compilation targets (see `--target` in `rustc -h`) for this
embedded target:

- armv7a-none-eabihf, hard float ABI; float operations are performed on the
  hardware FPU. NOTE: currently untested

- armv7a-none-eabi, soft float ABI; float operations are emulated

The default compilation target for the whole directory is currently set to the
most compatible soft float target. If you need to use the hard float target
modify `.cargo/config`, or pass `--target armv7a-none-eabihf` to Cargo.
