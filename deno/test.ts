import Context from "https://deno.land/std@0.178.0/wasi/snapshot_preview1.ts";
import { Memory } from "https://raw.githubusercontent.com/metatypedev/metatype/main/typegate/src/runtimes/python_wasi/memory.ts";
// import { Memory } from "./custom_memory.ts";

const path = "./build/python-wasi-reactor.wasm";

const context = new Context({
  env: {},
  args: [],
  preopens: {},
});

const binary = await Deno.readFile(path);
const module = new WebAssembly.Module(binary);
// const module = await WebAssembly.compile(binary);
const instance = new WebAssembly.Instance(module, {
  wasi_snapshot_preview1: {
    sock_accept(fd: any, flags: any) {
      return fd;
    },
    ...context.exports,
  },
  host: {
    callback: function(id: number, ptr: number) {
      const ret = memory.decode(ptr);
      if (ret.data) {
        console.log(ret.data);
        console.log("success callback", id, ":", JSON.parse(ret.data[0] as string));
      } else {
        console.log("error callback", id, ":", ret.error);
      }
    },
  },
});
const memory = new Memory(instance.exports);

context.initialize(instance);
console.log("exports:", Object.keys(instance.exports));

// init, apply and register are all in main.rs
// works similarly to bindgen
const { 
  init, 
  register_def,
  apply_def,
  unregister_def,
} = instance.exports as Record<
  string,
  CallableFunction
>;
init();


const tests = [
  {
    name: 'concat', 
    args: ['A', 'B', 1234],
    code: `
def concat(a, b, c):
  return f"Simple concat: {a}{b}{c}!"
    `
  },
  {
    name: 'sayHello', 
    args: [],
    code: `
def sayHello():
  return "Hello World"
    `
  },
]

// register
for (const {name, code} of tests) {
  const op = memory.decode(
    register_def(...memory.encode(name, code))
  );
  if (op.error) {
    console.log("python error", op.error);
    Deno.exit(1);
  }
}

// call
let id = 1;
for (const {name, args} of tests) {
  const ret = memory.decode(
    apply_def(...memory.encode(id++, name, JSON.stringify(args)))
  );
  console.log(ret);
  if (ret.data ) {
    console.log("exit status", ret.data[0]);
  } else {
      console.log("python error", ret.error);
  }
}