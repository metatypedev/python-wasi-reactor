#!/usr/bin/env bash

set -e

rm -rf build
mkdir build

cargo build --target wasm32-wasi -p python-wasi-reactor --release

./vendor/wasi-vfs/wasi-vfs \
    pack \
    target/wasm32-wasi/release/python-wasi-reactor.wasm \
    --mapdir /app::./src/python \
    --mapdir /usr/local/lib::./vendor/libpython/usr/local/lib \
    -o build/python-wasi-reactor.wasm 

if [[ "$1" = "--release" ]]; then
    wasm-opt -Oz build/python-wasi-reactor.wasm -o build/python-wasi-reactor.wasm
    tar cvzf build/python-wasi-reactor.tar.gz build/python-wasi-reactor.wasm
fi

