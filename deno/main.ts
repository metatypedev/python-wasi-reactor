import Context from "https://deno.land/std@0.178.0/wasi/snapshot_preview1.ts";
import { Memory } from "https://raw.githubusercontent.com/metatypedev/metatype/main/typegate/src/runtimes/python_wasi/memory.ts";
// import { Memory } from "../../metatype/typegate/src/runtimes/python_wasi/memory.ts";

const path = "./build/python-wasi-reactor.wasm";

const context = new Context({
  env: {},
  args: [],
  preopens: {},
});

const binary = await Deno.readFile(path);
const module = await WebAssembly.compile(binary);
const instance = new WebAssembly.Instance(module, {
  wasi_snapshot_preview1: {
    sock_accept(fd: any, flags: any) {
      return fd;
    },
    ...context.exports,
  },
  host: {
    callback(id: number, ptr: number) {
      const ret = memory.decode(ptr);
      if (ret.data) {
        console.log("success callback", id, ":", JSON.parse(ret.data[0]));
      } else {
        console.log("error callback", id, ":", ret.error);
      }
    },
  },
});
const memory = new Memory(instance.exports);

context.initialize(instance);
console.log("exports:", Object.keys(instance.exports));

const { init, apply_lambda, register_lambda } = instance.exports as Record<
  string,
  CallableFunction
>;
init();

const op = memory.decode(
  register_lambda(...memory.encode("foo", "lambda x:  x['a'] + 'a'"))
);

if (op.error) {
  console.log("python error", op.error);
  Deno.exit(1);
}

for (let i = 0; i < 3; i += 1) {
  console.time("foo");
  const ret = memory.decode(
    apply_lambda(...memory.encode(i, "foo", JSON.stringify({ a: `hello${i}` })))
  );
  if (ret.data) {
    console.log("exit status", i, ":", ret.data[0]);
  } else {
    console.log("python error", ret.error);
  }
  console.timeEnd("foo");
}
