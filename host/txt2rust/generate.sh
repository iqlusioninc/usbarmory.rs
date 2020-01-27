#!/bin/bash

set -euxo pipefail

main() {
    if [ $(md5sum $1 | cut -d' ' -f1) != 48a58957d65fbbde8602376d09776b5b ]; then
        echo 'md5sum check failed; you may need to update the start and end pages'
    fi

    pdftotext -layout -f 1141 -l 1142 $1 gpio.txt
    pdftotext -layout -f 2921 -l 2922 $1 snvs.txt
    pdftotext -layout -f 3101 -l 3107 $1 uart.txt
    pdftotext -layout -f 3584 -l 3584 $1 wdog.txt
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
