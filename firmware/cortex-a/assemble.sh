#!/bin/bash

set -euxo pipefail

main() {
    local pkg_name=cortex-a

    # NOTE: cflags taken from cc 1.0.49 / armv7-unknown-linux-gnueabi
    arm-none-eabi-as -march=armv7-a asm.s -o bin/$pkg_name.o
    ar crs bin/armv7-none-eabi.a bin/$pkg_name.o

    # TODO armv7-none-eabihf

    rm bin/*.o
}

main
