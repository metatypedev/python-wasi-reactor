#!/usr/bin/env bash

set -e

source dev/lock.sh

rm -rf build
mkdir build

cargo build --target wasm32-wasi --features "wasm" -p python-wasi-reactor --release

./vendor/wasi-vfs/wasi-vfs \
    pack \
    target/wasm32-wasi/release/python-wasi-reactor.wasm \
    --mapdir /app::./src/python \
    --mapdir /usr/local/lib::./vendor/libpython/usr/local/lib \
    -o build/python-wasi-reactor.wasm

if [[ "$1" = "--release" ]]; then
    OUT=python${PYTHON_VERSION}-wasi-reactor.wasm 
    mv build/python-wasi-reactor.wasm build/${OUT}
    wasm-opt -Oz build/${OUT} -o build/${OUT}
    tar cvzf build/${OUT}.tar.gz -C build ${OUT}
fi

