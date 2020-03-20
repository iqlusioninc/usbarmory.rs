#!/bin/bash

# how-to use: `./generate.sh IMX6ULRM.pdf IMX6ULLRM.pdf MCIMX28RM.pdf`

set -euxo pipefail

main() {
    local mx6ul=$1
    local mx6ull=$2
    local mx28=$3

    if [ $(md5sum $1 | cut -d' ' -f1) != 48a58957d65fbbde8602376d09776b5b ]; then
        echo 'md5sum check of the first PDF failed; you may need to update the start and end pages that are used in this script'
    fi

    if [ $(md5sum $2 | cut -d' ' -f1) != f7df5baacd27cce0c3fa4af9e36d4b3e ]; then
        echo 'md5sum check of the second PDF failed; you may need to update the start and end pages that are used in this script'
    fi

    if [ $(md5sum $3 | cut -d' ' -f1) != 6c743abe43b0ff99293ad0aa004f31ec ]; then
        echo 'md5sum check of the third PDF failed; you may need to update the start and end pages that are used in this script'
    fi

    rm -f *.txt
    pdftotext -layout -f 1081 -l 1082 $mx28 dcp.txt
    pdftotext -layout -f 1141 -l 1142 $mx6ul gpio.txt
    pdftotext -layout -f 1246 -l 1246 $mx6ul i2c.txt
    pdftotext -layout -f 1280 -l 1300 $mx6ul iomuxc.txt
    pdftotext -layout -f 2043 -l 2046 $mx6ul ccm-mmdc.txt
    pdftotext -layout -f 2921 -l 2922 $mx6ul snvs.txt
    pdftotext -layout -f 3093 -l 3093 $mx6ull rngb.txt
    pdftotext -layout -f 3101 -l 3107 $mx6ul uart.txt
    pdftotext -layout -f 3309 -l 3309 $mx6ul usb-nc.txt
    pdftotext -layout -f 3315 -l 3319 $mx6ul usb.txt
    pdftotext -layout -f 3403 -l 3405 $mx6ul usb-phy.txt
    pdftotext -layout -f 3420 -l 3421 $mx6ul usb-analog.txt
    pdftotext -layout -f 3504 -l 3506 $mx6ul usdhc.txt
    pdftotext -layout -f 3584 -l 3584 $mx6ul wdog.txt
    pdftotext -layout -f 641 -l 642 $mx6ul ccm.txt
    pdftotext -layout -f 693 -l 696 $mx6ul ccm-analog.txt
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
