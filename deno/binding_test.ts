// ❯ deno run -A --unstable test.ts
import { WasiReactorConfig, WasiReactorInp, run_wasi_func } from "../bindings/bindings.ts";

const config: WasiReactorConfig = {
    preopens: [
        "/app:./src/python:readonly"
    ],
    pylib_path: "./vendor/libpython/usr/local/lib",
    wasi_mod_path: "./build/python-wasi-reactor.wasm",
    reset_vm: false,
};

const input: WasiReactorInp = {
    callee: "identity",
    args: ["hello from identity"]
}

// use singleton
for (let i = 0; i < 5; i++) {
    const label = `test${i}`;
    console.time(label);
    console.log(run_wasi_func(input, {...config, reset_vm: false}));
    console.timeEnd(label);
}

console.log("------------------");
// use singleton
for (let i = 0; i < 5; i++) {
    const label = `test${i}`;
    console.time(label);
    console.log(run_wasi_func(input, {...config, reset_vm: true}));
    console.timeEnd(label);
}