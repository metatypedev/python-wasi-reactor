import { 
  apply_lambda, 
  register_virtual_machine, 
  register_lambda, PythonApplyInp, PythonRegisterInp
} from "../bindings/bindings.ts";
  

function promisify(fn: CallableFunction, args: unknown) {
  return new Promise((resolve, reject) => {
    try {
      resolve(fn(args));
    } catch (e) {
      reject(e);
    }
  });
}

const vmName = "myVm";
console.time("vmInit");
register_virtual_machine({
  vm_name: vmName,
  preopens: [
    "/app:./deno/python_scripts:readonly"
  ],
  pylib_path: "./vendor/libpython/usr/local/lib",
  wasi_mod_path: "./build/python-wasi-reactor.wasm",
});
console.timeEnd("vmInit");

await promisify(register_lambda, {
  name: "id",
  code: "lambda x: x['a']",
  vm: vmName
} as PythonRegisterInp);

const all = [...new Array(100)].map((_, n) => {
  return promisify(apply_lambda, {
    id: n,
    vm: vmName,
    name: "id",
    args: JSON.stringify([{a: "test"}])
  } as PythonApplyInp);
});

console.time("100 parallel");
await Promise.all(all);
console.timeEnd("100 parallel");

console.time("100 sequential");
for (let i = 0; i < 100; i++) {
  await promisify(apply_lambda, {
    id: i,
    vm: "myVm",
    name: "id",
    args: JSON.stringify([{a: "test"}])
  } as PythonApplyInp);
}
console.timeEnd("100 sequential");
