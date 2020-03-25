#!/bin/bash

set -euxo pipefail

main() {
    local crate=usbarmory-rt

    # NOTE: cflags taken from cc 1.0.49 / armv7-unknown-linux-gnueabi
    arm-none-eabi-as -march=armv7-a asm-common.s -o bin/$crate-common.o
    arm-none-eabi-as -march=armv7-a asm-dram.s -o bin/$crate-dram.o
    arm-none-eabi-as -march=armv7-a asm-ocram.s -o bin/$crate-ocram.o

    ar crs bin/armv7a-none-eabi-ocram.a bin/$crate-{common,ocram}.o
    ar crs bin/armv7a-none-eabi-dram.a bin/$crate-{common,dram}.o

    # arm-none-eabi-as -march=armv7-a+vfpv3 asm-common.s -o bin/$crate-common.o
    # arm-none-eabi-as -march=armv7-a+vfpv3 asm-vfp.s -o bin/$crate-vfp.o
    # ar crs bin/armv7a-none-eabihf.a bin/$crate-{common,vfp}.o

    rm bin/*.o
}

main
