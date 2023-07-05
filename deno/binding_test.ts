// ❯ deno run -A --unstable test.ts
import { assert } from "https://deno.land/std@0.190.0/testing/asserts.ts";
import { apply_lambda, register_virtual_machine, unregister_virtual_machine, register_module, apply_def,   } from "../bindings/bindings.ts";



const vm_name = "test_vm";
register_virtual_machine({
    vm_name,
    preopens: [
        "/app:./src/python:readonly"
    ],
    pylib_path: "./vendor/libpython/usr/local/lib",
    wasi_mod_path: "./build/python-wasi-reactor.wasm",
});


register_module({
    vm: vm_name,
    name: "my_mod",
    code: `
def say_hello(x, y):
    return f"Hello {x} and {y}"
`
})

for (let i = 0; i < 5; i++) {
    const label = `test${i}`;
    console.time(label);
    console.log(apply_def({
        vm: vm_name,
        id: 1,
        name: "my_mod.say_hello",
        args: JSON.stringify(["John", "Jake"]),
    }));
    console.timeEnd(label);
}

unregister_virtual_machine({vm_name});
const ret = apply_lambda({
    vm: vm_name,
    id: 1,
    name: "my_mod.say_hello",
    args: JSON.stringify(["John", "Jake"]),
});
assert((ret as any)?.["Err"]?.["message"] === "vm not initialized")