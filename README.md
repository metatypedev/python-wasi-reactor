# Python WASI reactor

> Python WASI reactor is part of the
> [Metatype ecosystem](https://github.com/metatypedev/metatype). Consider
> checking out how this component integrates with the whole ecosystem and browse
> the
> [documentation](https://metatype.dev?utm_source=github&utm_medium=readme&utm_campaign=python-wasi-reactor)
> to see more examples.

<details>
  <summary>What is WASM/WASI?</summary>

WebAssembly System Interface (WASI) is a standard interface for interacting with
system resources from WebAssembly (WASM) modules, providing a secure and
portable way to access low-level operating system functions.

</details>

This repository builds on the top of
[webassembly-language-runtimes](https://github.com/vmware-labs/webassembly-language-runtimes)
to provide a WASI Python runtime in reactor mode. In a reactor, the WASM guest
instance remains alive and and reacts to events from the host
([learn more](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-rationale.md#why-not-async)).

This is **experimental** and might not work as expected. Please report any
[issues](https://github.com/metatypedev/python-wasi-reactor/issues) you find or
[contribute](https://github.com/metatypedev/python-wasi-reactor/issues) back
improving the runtime.

## Getting started

[Wasmedge-bindgen](https://github.com/second-state/wasmedge-bindgen) has been
chosen over [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) because of
its focus on Rust as a host/guest.
[wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) may also be
interesting to consider in the future.

### Deno

[Deno](https://github.com/denoland/deno_std/blob/main/wasi/snapshot_preview1.ts)
does not yet provide a full implentation of WASI, yet the building blocks are
enough to run some workloads.

```bash
# increase stack size might be required on some cases: --v8-flags=--stack_size=3000
deno run -A --unstable deno/main.ts
```

### WasmEdge

[WasmEdge](https://github.com/WasmEdge/WasmEdge) has a custom implementation of
socket WASI API, which is not yet compatible with the project. The example will
be reworked once WasmEdge 0.12 will be
[released](https://github.com/WasmEdge/WasmEdge/issues/2056).

```bash
export DYLD_LIBRARY_PATH="$HOME/.wasmedge/lib:$DYLD_LIBRARY_PATH" # macOS
export LD_LIBRARY_PATH="$HOME/.wasmedge/lib:$LD_LIBRARY_PATH" # Linux
cargo run -p wasmedge
```

## Development

```bash
./install.sh
./build.sh

# enable optimization and compression
./build.sh --release
```
