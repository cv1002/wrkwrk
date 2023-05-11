#!/usr/bin/env sh

cargobuild() {
    if [[ $OSTYPE == 'darwin'* ]]; then
        cargo rustc $1 -- -C link-arg=-undefined -C link-arg=dynamic_lookup
    else
        cargo build $1
    fi
}

build() {
    rm -rf     build
    mkdir      build
    cargobuild --release
    cp         target/release/wrkwrk    build   
    chmod      777                      build/wrkwrk

    # Printout the size of output binary file.
    fileSize=$(du -kh build/wrkwrk | cut -f1)
    printf "total size: %s\n" $fileSize
}

clean() {
    rm -rf build
    rm -rf target
}

debugbuild() {
    rm -rf     build
    mkdir      build
    cargobuild
    cp         target/debug/wrkwrk    build   
    chmod      777                    build/wrkwrk

    # Printout the size of output binary file.
    fileSize=$(du -kh build/wrkwrk | cut -f1)
    printf "total size: %s\n" $fileSize
}

debugreleasebuild() {
    RUSTFLAGS=-g build
}

set -e

case $1 in
    "clean")
        clean
        ;;
    "build")
        build
        ;;
    "debugbuild")
        debugbuild
        ;;
    "debugreleasebuild")
        debugreleasebuild
        ;;
    "help" | *)
        ;;
esac
