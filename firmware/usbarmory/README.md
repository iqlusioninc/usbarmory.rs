# usbarmory.rs <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache 2.0 + MIT Licensed][license-image]
![MSRV][msrv-image]
[![Gitter Chat][gitter-image]][gitter-link]

Board support package for [USB armory MkII devices][usbarmory]
from [F-Secure].

<img src="https://storage.googleapis.com/iqlusion-production-web/github/usbarmory/usbarmory-mkII.png" alt="USB armory mkII" width="375" height="175">

## Minimum Supported Rust Version

- Rust **1.42**

## Status

This project is an incomplete work-in-progress in an early developmental
stage and will not be ready to use for some time.

## Building dependencies

- [Xargo](https://crates.io/crates/xargo), for the time being. `cargo install
xargo` (run this command *outside* the `firmware` directory)

## Development dependencies

- `arm-none-eabi-gcc`, only required if modifying assembly (`.s`) files

- `qemu-system-arm` v4.x, to run firmware on the host and for some unit testing.
  Install with `pacman -S qemu-arch-extra` on Arch Linux.

## Building examples

As the `armv7-none-eabi` target is not in `rustc` / `rustup` you'll have to use
Rust nightly and Xargo for now.

``` rust
$ # run this command from this directory
$ export RUST_TARGET_PATH=$(dirname `pwd`)

$ xargo build --example $example_name
```

*NOTE* You'll need to set the `RUST_TARGET_PATH` variable to use *any* Xargo
command that involves building, including `xargo run`.

## Running on QEMU

The examples whose name is prefixed with `qemu-` are meant to be run on QEMU and
not on hardware. To run these example use the following QEMU command:

(more details about QEMU & Rust, including debugging QEMU programs, can be found
in the [Embedded Rust book][book])

[book]: https://rust-embedded.github.io/book/start/qemu.html

``` console
$ qemu-system-arm \
  -cpu cortex-a7 \
  -machine mcimx6ul-evk  \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel $path_to_binary
```

Or simply run:

``` rust
$ xargo run --example $example_name
```

If a example doesn't explicitly terminate itself, press `C-a` + `c` to bring up
the QEMU console then enter the `quit` command to terminate QEMU.

## Contributing

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

## License

Copyright © 2020 iqlusion

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[build-image]: https://github.com/iqlusioninc/usbarmory.rs/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/usbarmory.rs/actions
[crate-image]: https://img.shields.io/crates/v/usbarmory.svg
[crate-link]: https://crates.io/crates/usbarmory
[docs-image]: https://docs.rs/usbarmory/badge.svg
[docs-link]: https://docs.rs/usbarmory/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/iqlusioninc/armistice/blob/develop/LICENSE
[msrv-image]: https://img.shields.io/badge/rustc-1.42+-red.svg
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[usbarmory]: https://inversepath.com/usbarmory.html
[F-Secure]: https://www.f-secure.com/
[CONTRIBUTING.md]: https://github.com/iqlusioninc/armistice/blob/develop/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/armistice/blob/develop/CODE_OF_CONDUCT.md
[Apache License, Version 2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT license]: https://opensource.org/licenses/MIT
