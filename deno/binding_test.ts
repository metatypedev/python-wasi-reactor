// ❯ deno run -A --unstable test.ts
import { WasiReactorConfig, WasiReactorInp, run_wasi_func } from "../bindings/bindings.ts";

const config: WasiReactorConfig = {
    preopens: [
        "/app:./src/python:readonly"
    ],
    pylib_path: "./vendor/libpython/usr/local/lib",
    wasi_mod_path: "./build/python-wasi-reactor.wasm"
};

const input: WasiReactorInp = {
    callee: "identity",
    args: ["hello from identity"]
}

const ret = run_wasi_func(input, config);
console.log(ret);