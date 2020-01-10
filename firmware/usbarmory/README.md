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
  Install it with `pacman -S qemu-arch-extra` on Arch Linux.

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
$ # working directory: usbarmory
$ qemu-system-arm \
  -cpu cortex-a7 \
  -machine mcimx6ul-evk  \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel ../target/armv7-none-eabi/release/examples/qemu-hello
```

Or simply run:

``` rust
$ xargo run --example qemu-hello
```

If a example doesn't explicitly terminate itself, press `C-a` + `c` to bring up
the QEMU console then enter the `quit` command to terminate QEMU.

## Running on hardware

### Requirements

- A micro SD card with a capacity of at least 4 GB

- USB Armory debug accessory

### ELF images

Some basic info about ELF images; feel free to skip this section if you now what
information an ELF file encodes.

When compiling a crate, `rustc` compiles Rust code down to a single ELF file (at
least when the target is not Windows or mac). This ELF file contains the *data*
that makes up the program (that is machine code, strings, initial values for
static variables, etc.) and *metadata* to indicates where this data should be
loaded when the programs starts executing.

> TODO binutils

### One time setup

We'll load ELF images interactively using the u-boot console so the first step
is to flash u-boot in some persistent memory to use it as the second stage
loader. We'll use the microSD card for this.

#### Flash u-boot

First flash [a pre-compiled Debian image][debian-images] (we have tested the
20191219 release) into the SD card. The steps for this are documented
[here][debian-flash]; a summary is reproduced below:

[debian-images]: https://github.com/inversepath/usbarmory-debian-base_image/releases
[debian-flash]: https://github.com/inversepath/usbarmory-debian-base_image#installation
 
``` rust
$ ls *.raw.xz
usbarmory-mark-two-debian_stretch-base_image-20191219.raw.xz
 
$ unxz *.raw.xz

$ # NOTE replace `/dev/sdX` with the path to the SD card
$ sudo dd if=*.raw of=/dev/sdX bs=1M conv=fsync

$ sync
```

Then insert the SD card in the Armory and set the boot switch (bottom side of
the PCB) to the `μSD` position. You may need to remove the enclosure as the boot
switch is covered by a plastic film.

#### Non-root access to the debug accessory

When plugged into a UNIX(-like) machine (via the side micro USB cable) the debug
accessory appears as 4 different TTY devices. On Linux, you'll see them as
`/dev/ttyUSB*` devices. 

``` console
$ # first: connect your PC to the debug accessory using a micro USB cable
$ ls -l /dev/ttyUSB*
crw-rw---- 1 root uucp 188, 0 Jan  9 13:06 /dev/ttyUSB0
crw-rw---- 1 root uucp 188, 1 Jan  9 13:06 /dev/ttyUSB1
crw-rw---- 1 root uucp 188, 2 Jan  9 13:06 /dev/ttyUSB2
crw-rw---- 1 root uucp 188, 3 Jan  9 13:06 /dev/ttyUSB3
```

The exact permissions depend on your OS / distribution. In the above case adding
yourself to `uucp` group (`usermod -a -G uucp $username` & log out and in) would
be all that's needed to access the USB device as a non-root user. If you need to
change permissions write a udev rule like the one below, which will make the TTY
devices world readable and writable:

``` console
$ cat /etc/udev/rules.d/50-usbarmory.rules
# USB Armory Mk II debug accessory (FT4232H)
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6011", MODE:="0666"

$ # make the above rule effective
$ # you'll need to unplug and replug the debug accessory
$ sudo udevadm control --reload-rules
```

#### Identify u-boot console

The Debian u-boot image is configured to use UART2 serial interface for its
console. The UART2 pins are routed to the *third* bus interface (*C*DBUS) of the
(FTDI chip on the) debug accessory. On Linux, this interface corresponds to the
`/dev/ttyUSB2` device. If your OS does not enumerate the USB devices of the FTDI
chip in order then you can connect to each device in turn to see which is used
for the u-boot console and Linux logs.

Any terminal emulator will do (e.g. `ckermit`, `picocom`, etc.) but I recommend
using `minicom` as we'll use that in the next step. Create the following file to
make the defaults settings of `minicom` compatible with the Debian u-boot
console. 

> NOTE: on Arch Linux you can install `minicom` with the following command `sudo
> pacman -S minicom`

> IMPORTANT: the following file must end in a newline or `minicom` will hang (!)
> on startup

``` console
$ cat ~/.minirc.dfl
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No

```

Then you can connect to a USB serial interface using the following command:

``` console
$ minicom -D /dev/ttyUSB2
```

Now connect the debug accessory to the *side* USB-C port of the Armory; make
sure that the top sides of both boards are lined up: the LEDs on the Armory
should be facing up and the components of the debug accessory should also be
facing up. Note that it's *not* necessary to connect the Armory itself to your
PC. If you power cycle the Armory by unplugging and replugging the debug
accessory to your PC (micro USB cable) then you should see this output on
`minicom` -- if you have connected to the right TTY device, otherwise repeat the
power cycling with a different TTY device.


``` console
U-Boot 2019.07 (Dec 19 2019 - 12:39:10 +0100)

CPU:   Freescale i.MX6ULL rev1.1 at 396 MHz
Reset cause: WDOG
Model: F-Secure USB armory Mk II
Board: F-Secure USB armory Mk II
I2C:   ready
DRAM:  512 MiB
MMC:   FSL_SDHC: 0, FSL_SDHC: 1
Loading Environment from MMC... OK
In:    serial
Out:   serial
Err:   serial
Net:   usb_ether
Error: usb_ether address not set.

Hit any key to stop autoboot:  0
switch to partitions #0, OK
mmc0 is current device
Scanning mmc 0:1...
switch to partitions #0, OK
mmc1(part 0) is current device
** No partition table - mmc 1 **
3065816 bytes read in 151 ms (19.4 MiB/s)
22311 bytes read in 22 ms (990.2 KiB/s)
Kernel image @ 0x80800000 [ 0x000000 - 0x2ec7d8 ]
## Flattened Device Tree blob at 82000000
   Booting using the fdt blob at 0x82000000
   Loading Device Tree to 9f567000, end 9f56f726 ... OK

Starting kernel ...
(..)
Debian GNU/Linux 9 usbarmory ttymxc1

usbarmory login:
```

Now you know which TTY device to use!

#### Disable the Linux startup

The Debian u-boot defaults to loading the Linux kernel but we want access to the
u-boot console for development so let's change that. You can interrupt the Linux
startup by entering the escape key into the u-boot console within two (!)
seconds of the boot process. So have a `minicom` session connected the right TTY
device, then power cycle the Armory and immediately mash the escape key on your
keyboard. If you were fast enough you should see this output in the `minicom`
session: 

``` console
=>
```

`=> ` is the u-boot prompt. If you don't see anything print you likely
successfully stopped the Linux startup; you can confirm by entering the
following command into `minicom`:

``` console
=> version
U-Boot 2019.07 (Dec 19 2019 - 12:39:10 +0100)

arm-linux-gnueabihf-gcc (Debian 8.3.0-2) 8.3.0
GNU ld (GNU Binutils for Debian) 2.31.1
```

Now enter the `printenv` command into the u-boot console:

``` console
=> printenv
(..)
bootcmd=run start_normal
bootcmd_mmc0=devnum=0; run mmc_boot
bootcmd_mmc1=devnum=1; run mmc_boot
bootdelay=2
(..)
```

Check the `bootcmd` variable. This is the default boot command so write down its
contents somewhere if you want to restore it later because we'll change it to
*not* boot Linux. Run the following commands to change the boot command:

``` console
=> setenv bootcmd version

=> saveenv
Saving Environment to MMC... Writing to MMC(0)... OK

=> reset
resetting ...

U-Boot 2019.07 (Dec 19 2019 - 12:39:10 +0100)

CPU:   Freescale i.MX6ULL rev1.1 at 396 MHz
Reset cause: WDOG
Model: F-Secure USB armory Mk II
Board: F-Secure USB armory Mk II
I2C:   ready
DRAM:  512 MiB
MMC:   FSL_SDHC: 0, FSL_SDHC: 1
Loading Environment from MMC... OK
In:    serial
Out:   serial
Err:   serial
Net:   usb_ether
Error: usb_ether address not set.

Hit any key to stop autoboot:  0
U-Boot 2019.07 (Dec 19 2019 - 12:39:10 +0100)

arm-linux-gnueabihf-gcc (Debian 8.3.0-2) 8.3.0
GNU ld (GNU Binutils for Debian) 2.31.1

=> 
```

Now the Armory will always boot to the u-boot console.

#### Setting up `ckermit`

We'll transfer ELF images from the host to the Armory using the serial
connection (u-boot console) between the two. The u-boot project recommends the
C-Kermit terminal emulator for these binary transfers so let's set that up.

> NOTE: On Arch Linux, you can install the tool with the `sudo pacman -S
> ckermit` command. 

Create the following file to set the default settings of the tool:

> NOTE: These are the settings recommended in the [u-boot manual]

[u-boot manual]: https://www.denx.de/wiki/view/DULG/SystemSetup#Section_4.3.

``` console
$ cat .kermrc
set carrier-watch off
set handshake none
set flow-control none
robust
set file type bin
set file name lit
set rec pack 1000
set send pack 1000
set window 5
```

### Loading an ELF image

> TODO

Once you are done configuring u-boot on the target and C-Kermit on the host use
the following procedure to load ELF images:

> NOTE: on Arch Linux, the `/var/lock` directory is not writable by non-root
> users so the `ckermit` command will fail to start. A workaround is to use a
> different lock directory using the `LOCK_DIR` env variable. For example,
> setting it like this: `export LOCK_DIR=/tmp` does the trick.

``` console
$ ckermit -l /dev/ttyUSB2 -b 115200 -c
```

This will establish a serial connection to the target; this is the u-boot
console we used before:

``` console
=> version
U-Boot 2019.07 (Dec 19 2019 - 12:39:10 +0100)

arm-linux-gnueabihf-gcc (Debian 8.3.0-2) 8.3.0
GNU ld (GNU Binutils for Debian) 2.31.1
```

The first step is to load the *entire* ELF image into the target memory. ELF
images contain metadata and debug symbols so they can be rather large (e.g.
MBs). We'll load the ELF image into DRAM (DDR3 RAM) first using the `loadb`
command: 

> NOTE DRAM starts at address `0x8000_0000` and it has a size of 512 MB (you can
> check these facts using the `bdinfo` command in the u-boot console); part of
> this memory is used by u-boot itself. `loadb` with no arguments will load data
> into a part of DRAM that's not used by u-boot

``` console
=> loadb
## Ready for binary (kermit) download to 0x82000000 at 115200 bps...
```

Now press `Ctrl-\`, followed by `c`. This will bring you to the C-Kermit
console:

``` console
=> loadb
## Ready for binary (kermit) download to 0x82000000 at 115200 bps...

(Back at x1)
----------------------------------------------------
C-Kermit 9.0.302 OPEN SOURCE:, 20 Aug 2011, for Linux (64-bit)
 Copyright (C) 1985, 2011,
  Trustees of Columbia University in the City of New York.
Type ? or HELP for help.
(/tmp) C-Kermit>
```

Then tell C-Kermit where to find the ELF image using the `send` command:

``` console
(/tmp) C-Kermit> send /a/b/usbarmory.rs/firmware/target/armv7-none-eabi/release/examples/leds
```

You'll briefly see a progress bar and then return to the C-Kermit console. Now
enter the `connect` command to return to the u-boot console:

``` console
(/tmp) C-Kermit> connect
Connecting to /dev/ttyUSB2, speed 115200
 Escape character: Ctrl-\ (ASCII 28, FS): enabled
Type the escape character followed by C to get back,
or followed by ? to see other options.
----------------------------------------------------
CACHE: Misaligned operation at range [90000000, 90001284]
## Total Size      = 0x00001284 = 4740 Bytes
## Start Addr      = 0x82000000
=> 
```

Now we'll tell u-boot to read the ELF image we just wrote in DRAM, load the
program data into the memory regions specified by the ELF metadata, and run the
program by jumping into the entry point of the ELF. All this is done with the
`bootelf` command:

``` console
$ bootelf -s
## Starting application at 0x00900024 ...
```

If you run the `leds` example you should see the blue LED turn on and the white
LED turn off.

"One-off" applications, like `leds`, will reset the SoC and return you to the
u-boot console; from there you can load a new program using `loadb` and
`bootelf` as shown before.

Some applications, like `blinky`, have "no end" and won't reset the board. To
flash a new program after running these applications you'll have to power cycle
the Armory (unplug and replug) and repeat these steps. 

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
