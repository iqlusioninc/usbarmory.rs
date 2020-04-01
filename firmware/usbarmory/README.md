# usbarmory.rs <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache 2.0 + MIT Licensed][license-image]
![MSRV][msrv-image]
[![Gitter Chat][gitter-image]][gitter-link]

Board support package for [USB armory Mk II devices][usbarmory]
from [F-Secure].

<img src="https://storage.googleapis.com/iqlusion-production-web/github/usbarmory/usbarmory-mkII.png" alt="USB armory mkII" width="375" height="175">

## Minimum Supported Rust Version

- Rust **1.42**

## Status

This project is an incomplete work-in-progress in an early developmental
stage and will not be ready to use for some time.

## Building dependencies

- [flip-lld], linker wrapper that adds zero-cost stack overflow protection.
  `cargo install --git https://github.com/japaric/flip-lld`.

- `armv7a-none-eabi` compiler support. Install with `rustup target add
  armv7a-none-eabi`.

## Development dependencies

- [`imx_usb`] to load Rust programs directly into RAM. This program is available
  on Arch Linux as the `imx-usb-loader-git` AUR package.

[`imx_usb`]: https://github.com/boundarydevices/imx_usb_loader. *This depedency doesn't appear to be able to claim the USB device on MacOS. Currently the best solution is VirtualBox to a Linux guest. Remember to install Guest Additions!*

- `arm-none-eabi-binutils` OR (`cargo-binutils` + `llvm-tools-preview`), if you
  need to inspect ELF files. `sudo pacman -S arm-none-eabi-binutils` (Arch
  Linux) for the former; `cargo install cargo-binutils` (run it outside the
  `firmware` directory) and `rustup component add llvm-tools-preview` for the
  latter.

- `arm-none-eabi-gcc`, required to load programs into the eMMC. Also needed when
  modifying assembly (`.s`) files (these need to be re-assembled) which can be acquired [here](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).

- `qemu-system-arm` v4.x, to run firmware on the host and for some unit testing.
  Install it with `sudo pacman -S qemu-arch-extra` on Arch Linux.

## Building examples

``` rust
$ # on this directory
$ cargo build --example $example_name
```

## Running on QEMU

> NOTE: QEMU examples are currently broken (they need a different pre-main
> initialization routine)

The examples whose name is prefixed with `qemu-` are meant to be run on QEMU and
not on hardware. To run these example use the following QEMU command:

(more details about QEMU & Rust, including debugging QEMU programs, can be found
in the [Embedded Rust book][book])

[book]: https://rust-embedded.github.io/book/start/qemu.html

``` console
$ # working directory: usbarmory
$ qemu-system-arm \
  -cpu cortex-a7 \
  -machine mcimx6ul-evk  \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel ../target/armv7a-none-eabi/release/examples/qemu-hello
```

If a example doesn't explicitly terminate itself, press `C-a` + `c` to bring up
the QEMU console then enter the `quit` command to terminate QEMU.

## Good to know

### ELF images

Some basic info about ELF images; feel free to skip this section if you now what
information an ELF file encodes.

When compiling a crate, `rustc` compiles Rust code down to a single ELF file (at
least when the target is not Windows or mac). This ELF file contains the *data*
that makes up the program (that is machine code, strings, initial values for
static variables, etc.) and *metadata* to indicates where this data should be
loaded when the programs starts executing.

Because of metadata the size of ELF files on disk does not accurately reflect
the size of the program once it's loaded in memory. To see the real size of the
program you need to run the `size` program on the ELF file.

``` rust
$ cargo build --example hello --release

$ stat --printf="%s\n" ../target/armv7a-none-eabi/release/examples/hello
40104

$ # if you installed `arm-none-eabi-binutils`
$ arm-none-eabi-size -Ax ../target/armv7a-none-eabi/release/examples/hello
../target/armv7a-none-eabi/release/examples/hello  :
section             size       addr
.text              0x564   0x91f820
.rodata            0x27b   0x91fd84
.data                0x0   0x91ffff
.bss                 0x1   0x91ffff

$ # if you installed `cargo-binutils`
$ cargo size --release --example hello -- -A
hello  :
section              size      addr
.text                1380  0x91f820
.rodata               635  0x91fd84
.data                   0  0x91ffff
.bss                    1  0x91ffff
```

The ELF file is 40 KB on disk but only uses around 2 KB of memory when loaded
in RAM.

### Using the USB Armory debug accessory

You can omit this section if you are not going to use the debug accessory.

The debug accessory lets you receive data sent by the Armory through its serial
interface (UART). To visualize the data and interact with the Armory via the
serial interface you'll need to install and configure a terminal emulator like
`minicom`. Furthermore, on Linux (and other POSIX OSes) you'll need to tweak
some permissions to open the serial device as a non-root user. This section
covers how to do those two.

#### `minicom`

Install `minicom` using the following command (Arch Linux):

``` console
$ sudo pacman -S minicom
```

Create the following configuration file:

> **IMPORTANT** this file must contain at least one newline (`\n`) at the end or
> `minicom` will fail to start, parse the config file or hang.

```
$ cat ~/.minirc.dfl
pu addcarreturn Yes
pu baudrate 115200
pu bits 8
pu parity N
pu rtscts No
pu stopbits 1
pu xonxoff No

```

#### Non-root permissions

If you connect the debug accessory to your PC using a micro USB cable you'll see
the following USB device in `lsusb` (or equivalent):

``` console
$ lsusb
(..)
Bus 001 Device 029: ID 0403:6011 Future Technology Devices International (..)
```

On Linux, to use this USB device as a non-root user create the following file:

``` console
$ cat /etc/udev/rules.d/50-usbarmory.rules
ATTRS{idVendor}=="15a2", ATTRS{idProduct}=="0080", TAG+="uaccess"
```

Then run the following command to update `udev` rules:

``` console
$ sudo udevadm control --reload-rules
```

Re-connecting the debug accessory to your PC should show the following
permissions: 

``` console
$ lsusb
(..)
Bus 001 Device 030: ID 0403:6011 Future Technology Devices International (..)

$ # the '+' is the important part
$ ls -l /dev/bus/usb/001/030
crw-rw----+ 1 root root 189, 28 Mar 24 12:32 /dev/bus/usb/001/029
```

If you don't get a similar output try logging out and logging in again.

## Running on hardware (development mode)

### Required hardware

- USB Armory debug accessory
- micro USB cable (for the debug accessory)

### One time setup

To be able to use the Cargo runner to quickly load and run Rust programs you'll
need to zero the program image currently stored in the internal eMMC. Follow the
steps of "Setting up an eMMC Boot" up to *before* the `dd` invocation. Instead
of flashing a image proceed to zero the 4 KB that follow the 1 KB padding
(reserved for the partition table).

**WARNING** the following command will corrupt existing data. Do a backup if
there's anything on the eMMC that you'll like to keep.

``` console
$ sudo dd if=/dev/zero of=/dev/sda bs=512 seek=2 count=8 conv=fsync
$ sync
```

Now terminate the USB Mass Store Device emulation by pressing Ctrl-C in the
`minicom` terminal / u-boot console. You can now disconnect the Armory.

### Hardware configuration (boot mode)

- Select the microSD as the Armory's boot mode: there's a DIP switch on the back
of the Armory; put it in the "eMMC" position.

Now connect the Armory to the PC (USB-C port). If you run `lsusb` (or
 equivalent) you should see the following Vendor ID -  Product ID pair:

``` console
$ lsusb
(..)
Bus 001 Device 022: ID 15a2:0080 Freescale Semiconductor, Inc.
(..)
```

### Loading an ELF image

Navigate to the `firmware/usbarmory` and use `cargo run` to load a program into
memory. 

**IMPORTANT** Set the `COLD_BOOT` environment variable when loading a program
for the *first* time after power cycling the board (unplugging it and plugging
it again). Omit the env var in consecutive invocations.

``` console
$ # use COLD_BOOT the first time

$ COLD_BOOT=1 cargo run --example hello
Hello, world!
(device has reset)

$ # then omit COLD_BOOT

$ cargo run --example hello
Hello, world!
(device has reset)
```

Programs loaded using the Cargo runner are loaded into RAM. These programs will
be lost when power is removed from the board.

## Setting up a uSD Boot

### Required hardware

- uSD card
- (USB) uSD card reader (host side)

### Creating a program image

The ROM bootloader doesn't understand ELF files; it expects a different program
image format. A tool, named `elf2image`, is provided, in `host/image`, to
convert ELF files to the image format expected by the ROM bootloader.

Use the following command, from the `host/image` directory, to convert an ELF
file: 

``` console
$ # adjust this to your needs
$ path2elf=../../firmware/target/armv7a-none-eabi/debug/examples/blinky

$ cargo run --bin elf2image -- $path2elf

$ stat --printf="%s\n" blinky.bin
9840

$ hexyl -n16 blinky.bin
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ d1 00 20 40 00 08 00 80 ┊ 00 00 00 00 2c 04 00 80 │×0 @0•0×┊0000,•0×│
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
```

### Flashing the image

Insert the uSD card into your PC's card reader and identify its device file.

``` console
$ lsblk
NAME          MAJ:MIN RM   SIZE RO TYPE  MOUNTPOINT
mmcblk0       179:0    0  14.5G  0 disk
```

Run the following command to flash the image into the uSD card.

**WARNING** the following command will corrupt existing data. Do a backup if
there's anything on the uSD that you'll like to keep.

**NOTE** The ROM bootloader expects the boot image to be located at a
1024-byte offset. Hence the `seek=2` argument in the previous command. This
also means that you can keep a partition table in those first 1024 bytes (e.g.
MBR). You can partition the uSD before running the `dd` command -- ensure any
partition you create doesn't collide with the image you are about to flash.

``` console
$ sudo dd if=blinky.bin of=/dev/mmcblk0 bs=512 seek=2 conv=fsync
$ sync
```

### Hardware configuration (boot mode)

- Select the microSD as the Armory's boot mode: there's a DIP switch on the back
of the Armory; put it in the "uSD" position.

- *Insert* the uSD card into the Armory's slot.

Now you can plug the Armory into a USB-C port and it will run the program you
just flashed.

## Setting up an eMMC Boot

As the Armory HAL currently doesn't provide functionality to receive images from
a PC and flash them into the internal eMMC we'll use [u-boot] to flash images
into the eMMC.

[u-boot]: https://www.denx.de/wiki/U-Boot

### Required hardware

- USB Armory debug accessory
- micro USB cable (for the debug accessory)

**NOTE** if you haven't used the debug accessory before check out the "Running
on hardware (development mode)" section for details on how to configure your PC
to use it.

### Creating a program image

Same steps as in the "Setting up a uSD Boot" version.

### Building U-Boot

Clone U-Boot from `https://gitlab.denx.de/u-boot/u-boot.git` and check out the
[`v2019.07`] tag, and obtain the following patches to add support for the USB
Armory Mk II:

* [0001-ARM-mx6-add-support-for-USB-armory-Mk-II-board.patch][ubootpatch0]
* [0001-Drop-linker-generated-array-creation-when-CONFIG_CMD.patch][ubootpatch1]

Now apply both of them by running `git am < file.patch` while in the checked-out
repository.

Run `make usbarmory-mark-two_config` to use the default USB Armory
configuration.

Then run `make`, as shown below, to build U-Boot:

Linux
``` console
$ # NOTE this depends on arm-none-eabi-gcc and other C build tools
$ ARCH=arm CROSS_COMPILE=arm-none-eabi- make
```

On Mac, you need to work around two things.

1. You need to make the compiler and linker aware of OpenSSL. These [instructions](https://medium.com/@timmykko/using-openssl-library-with-macos-sierra-7807cfd47892) worked.

2. Need to work around MacOS variant of `sed`. [This](https://gist.github.com/andre3k1/e3a1a7133fded5de5a9ee99c87c6fa0d) is one option.

This should result in a `u-boot-dtb.imx` file, which contains the built U-Boot
binary.

[`v2019.07`]: https://github.com/u-boot/u-boot/releases/tag/v2019.07
[ubootpatch0]: https://raw.githubusercontent.com/f-secure-foundry/usbarmory/master/software/u-boot/0001-ARM-mx6-add-support-for-USB-armory-Mk-II-board.patch
[ubootpatch1]: https://raw.githubusercontent.com/f-secure-foundry/usbarmory/master/software/u-boot/0001-Drop-linker-generated-array-creation-when-CONFIG_CMD.patch

### Flashing the image

For this step you'll need to put the Armory in uSD boot mode (dip switch on
the back set to "uSD") and *remove* any uSD card from the Armory's card slot.

Now connect the debug accessory to the USB Armory, connect the USB Armory into
one of your PC's USB-C ports and connect the debug accessory to your PC (micro
USB cable). You should see the following USB devices appear in `lsusb` (or
equivalent): 

``` console
$ lsusb
(..)
Bus 001 Device 025: ID 15a2:0080 Freescale Semiconductor, Inc.
(..)
Bus 001 Device 029: ID 0403:6011 Future Technology Devices International (..)
```

Now load u-boot into the device's RAM using the following command:

``` console
$ imx_usb -v ./u-boot-dtb.imx
(..)
<<<531456, 531456 bytes>>>
succeeded (security 0x56787856, status 0x88888888)
Verify success
jumping to 0x877ff400
```

Now access the u-boot console using `minicom`:

``` console
$ minicom -b 115200 -D /dev/ttyUSB2

=> version
U-Boot 2019.07-00002-gba935971cd (Mar 24 2020 - 12:23:32 +0100)

arm-none-eabi-gcc (Arch Repository) 9.2.0
GNU ld (GNU Binutils) 2.34
```

Run the following command in the u-boot console to expose the eMMC as a USB Mass
Storage Device:

``` console
=> ums 0 mmc 1
```

On a new host terminal identify the USB device:

``` console
$ lsblk
NAME          MAJ:MIN RM   SIZE RO TYPE  MOUNTPOINT
sda             8:0    1  14.6G  0 disk
```

Flash the program image (the output of `elf2image`) using the following command:

**WARNING** the following command will corrupt existing data. Do a backup if
there's anything on the eMMC that you'll like to keep.

**NOTE** The ROM bootloader expects the boot image to be located at a
1024-byte offset. Hence the `seek=2` argument in the previous command. This
also means that you can keep a partition table in those first 1024 bytes (e.g.
MBR). You can partition the uSD before running the `dd` command -- ensure any
partition you create doesn't collide with the image you are about to flash.

Linux

``` console
$ sudo dd if=blinky.bin of=/dev/sda bs=512 seek=2 conv=fsync
$ sync
```

MacOS

``` console
$ sudo dd if=blinky.bin of=/dev/rdisk2 bs=512 seek=2 conv=sync
$ sync
```


Now terminate the USB Mass Store Device emulation by pressing Ctrl-C in the
`minicom` terminal / u-boot console. You can now disconnect the Armory.

### Hardware configuration (boot mode)

- Select the eMMC as the Armory's boot mode: there's a DIP switch on the back
  of the Armory; put it in the "eMMC" position.

Now you can plug the Armory into a USB-C port and it will run the program you
just flashed.

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

[usbarmory]: https://github.com/f-secure-foundry/usbarmory/wiki
[F-Secure]: https://foundry.f-secure.com/
[CONTRIBUTING.md]: https://github.com/iqlusioninc/armistice/blob/develop/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/armistice/blob/develop/CODE_OF_CONDUCT.md
[Apache License, Version 2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT license]: https://opensource.org/licenses/MIT
