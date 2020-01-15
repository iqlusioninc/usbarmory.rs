#!/bin/bash

set -euxo pipefail

main() {
    local crate=usbarmory-rt

    # NOTE: cflags taken from cc 1.0.49 / armv7-unknown-linux-gnueabi
    arm-none-eabi-as -march=armv7-a asm-common.s -o bin/$crate-common.o
    arm-none-eabi-as -march=armv7-a asm-no-vfp.s -o bin/$crate-no-vfp.o
    ar crs bin/armv7-none-eabi.a bin/$crate-{common,no-vfp}.o

    arm-none-eabi-as -march=armv7-a+vfpv3 asm-common.s -o bin/$crate-common.o
    arm-none-eabi-as -march=armv7-a+vfpv3 asm-vfp.s -o bin/$crate-vfp.o
    ar crs bin/armv7-none-eabihf.a bin/$crate-{common,vfp}.o

    rm bin/*.o
}

main
