#!/usr/bin/env bash

set -e

WASI_SDK_VERSION=wasi-sdk-19

WASI_VFS_VERSION=v0.2.0
WASI_VFS_DL=https://github.com/kateinoigakukun/wasi-vfs/releases/download/${WASI_VFS_VERSION}/libwasi_vfs-wasm32-unknown-unknown.zip

LIBPYTHON_VERSION=libpython-3.11.1
LIBPYTHON_DL=https://github.com/assambar/webassembly-language-runtimes/releases/download/python%2F3.11.1%2B20230223-8a6223c/libpython-3.11.1.tar.gz

rm -rf vendor
mkdir vendor

case "$(uname -s)" in
    Linux*)     
        WASI_SDK_DL=https://github.com/WebAssembly/wasi-sdk/releases/download/${WASI_SDK_VERSION}/${WASI_SDK_VERSION}.0-linux.tar.gz
        WASI_VFS_CLI_DL=https://github.com/kateinoigakukun/wasi-vfs/releases/download/${WASI_VFS_VERSION}/wasi-vfs-cli-x86_64-unknown-linux-gnu.zip
        ;;
    Darwin*)    
        WASI_SDK_DL=https://github.com/WebAssembly/wasi-sdk/releases/download/${WASI_SDK_VERSION}/${WASI_SDK_VERSION}.0-macos.tar.gz
        WASI_VFS_CLI_DL=https://github.com/kateinoigakukun/wasi-vfs/releases/download/${WASI_VFS_VERSION}/wasi-vfs-cli-aarch64-apple-darwin.zip
        ;;
    *)          echo "Unknown OS"; exit 1
esac

curl -fsSL ${WASI_SDK_DL} -o vendor/${WASI_SDK_VERSION}.tar.gz
tar -xf vendor/${WASI_SDK_VERSION}.tar.gz -C vendor
mv vendor/${WASI_SDK_VERSION}.0 vendor/wasi-sdk

mkdir -p vendor/${LIBPYTHON_VERSION}
curl -fsSL ${LIBPYTHON_DL} -o vendor/${LIBPYTHON_VERSION}.tar.gz
tar -xf vendor/${LIBPYTHON_VERSION}.tar.gz -C vendor/${LIBPYTHON_VERSION}
mv vendor/${LIBPYTHON_VERSION} vendor/libpython

mkdir -p vendor/wasi-vfs/lib
curl -fsSL ${WASI_VFS_CLI_DL} -o vendor/wasi-vfs-cli-${WASI_VFS_VERSION}.zip
unzip vendor/wasi-vfs-cli-${WASI_VFS_VERSION}.zip -d vendor/wasi-vfs

curl -fsSL ${WASI_VFS_DL} -o vendor/wasi-vfs-${WASI_VFS_VERSION}.zip
unzip vendor/wasi-vfs-${WASI_VFS_VERSION}.zip -d vendor/wasi-vfs/lib

