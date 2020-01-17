#!/bin/bash

set -euxo pipefail

main() {
    if [ $(md5sum RM.pdf | cut -d' ' -f1) != 48a58957d65fbbde8602376d09776b5b ]; then
        echo 'md5sum check failed; you may need to update the start and end pages'
    fi

    pdftotext -layout -f 1141 -l 1142 RM.pdf gpio.txt
    pdftotext -layout -f 2921 -l 2922 RM.pdf snvs.txt
    cargo run -- gpio.txt snvs.txt

    local td=$(mktemp -d)
    cargo init --lib $td --name pac
    mv lib.rs $td/src/
    pushd $td/src
    cargo fmt
    cargo check
    # cargo doc --open # NOTE if you use this then do NOT delete the tempdir
    popd
    rm -rf $td
}

main
