#!/bin/bash

set -euxo pipefail

main() {
    local crate=usbarmory-rt

    # NOTE: cflags taken from cc 1.0.49 / armv7-unknown-linux-gnueabi
    arm-none-eabi-as -march=armv7-a asm.s -o bin/$crate.o
    ar crs bin/armv7-none-eabi.a bin/$crate.o

    # TODO armv7-none-eabiHF

    rm bin/*.o
}

main
