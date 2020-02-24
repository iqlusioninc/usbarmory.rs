#!/bin/bash

# how-to use: `./generate.sh IMX6ULRM.pdf IMX6ULLRM.pdf`

set -euxo pipefail

main() {
    local ulrm=$1
    local ullrm=$1

    if [ $(md5sum $1 | cut -d' ' -f1) != 48a58957d65fbbde8602376d09776b5b ]; then
        echo 'md5sum check of the first PDF failed; you may need to update the start and end pages that are used in this script'
    fi

    if [ $(md5sum $2 | cut -d' ' -f1) != f7df5baacd27cce0c3fa4af9e36d4b3e ]; then
        echo 'md5sum check of the second PDF failed; you may need to update the start and end pages that are used in this script'
    fi

    rm -f *.txt
    pdftotext -layout -f 1141 -l 1142 $1 gpio.txt
    pdftotext -layout -f 2921 -l 2922 $1 snvs.txt
    pdftotext -layout -f 3093 -l 3093 $2 rngb.txt
    pdftotext -layout -f 3101 -l 3107 $1 uart.txt
    pdftotext -layout -f 3309 -l 3309 $1 usb-nc.txt
    pdftotext -layout -f 3315 -l 3319 $1 usb.txt
    pdftotext -layout -f 3403 -l 3405 $1 usb-phy.txt
    pdftotext -layout -f 3420 -l 3421 $1 usb-analog.txt
    pdftotext -layout -f 3504 -l 3506 $1 usdhc.txt
    pdftotext -layout -f 3584 -l 3584 $1 wdog.txt
    pdftotext -layout -f 693 -l 696 $1 ccm-analog.txt
    cargo run -- *.txt

    local crate=imx6ul-pac
    local pac=../../firmware/$crate
    mv lib.rs $pac/src/
    pushd $pac
    cargo fmt
    cargo check
    popd
}

main "${@}"
