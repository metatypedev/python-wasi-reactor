import Context from "https://deno.land/std@0.178.0/wasi/snapshot_preview1.ts";

const binary = await Deno.readFile("./build/python-wasi-reactor.wasm");

const module = await WebAssembly.compile(binary);

const context = new Context({
  env: {},
  args: [],
  preopens: {},
});

const instance = new WebAssembly.Instance(module, {
  wasi_snapshot_preview1: {
    sock_accept(fd, flags) {
      return fd;
    },
    ...context.exports,
  },
});

console.log("start");
console.log(instance.exports);

context.initialize(instance);

const encoder = new TextEncoder();
const decoder = new TextDecoder();

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };
type XOR<T, U> = T | U extends object
  ? (Without<T, U> & U) | (Without<U, T> & T)
  : T | U;

class Memory {
  private memory: WebAssembly.Memory;
  private allocate: CallableFunction;
  private deallocate: CallableFunction;

  constructor(exports: WebAssembly.Exports) {
    this.memory = exports.memory as WebAssembly.Memory;
    this.allocate = exports.allocate as CallableFunction;
    this.deallocate = exports.deallocate as CallableFunction;
  }

  encode(...args): [number, number] {
    const size = args.length;
    const ptr = this.allocate(size * 2 * 4);
    const view = new DataView(this.memory.buffer, ptr, size * 2 * 4);

    for (let i = 0; i < size; i += 1) {
      if (typeof args[i] == "string") {
        const bytes = encoder.encode(args[i]);
        const value = this.allocate(bytes.length);
        const viewV = new DataView(this.memory.buffer, value, bytes.length);
        for (let j = 0; j < bytes.length; j += 1) {
          viewV.setUint8(j, bytes[j]);
        }
        view.setInt32(i * 2 * 4, value, true);
        view.setInt32(i * 2 * 4 + 4, bytes.length, true);
      } else {
        throw new Error("not implemented");
      }
    }

    return [ptr, size];
  }

  decode(n: number): XOR<{ data: any[] }, { error: string }> {
    const ret = new DataView(this.memory.buffer, n, 9);
    const status = ret.getInt8(0);
    const ptr = ret.getInt32(1, true);
    const size = ret.getInt32(5, true);
    this.deallocate(n, 9);

    if (status == 1) {
      // error
      const value = decoder.decode(this.memory.buffer.slice(ptr, ptr + size));
      this.deallocate(ptr, size);
      return { error: value };
    }

    const p_data = new DataView(this.memory.buffer, ptr, size * 3 * 4);
    let p_values: number[] = [];
    for (let i = 0; i < size * 3; i += 1) {
      p_values[i] = p_data.getInt32(i * 4, true);
    }

    const values: any[] = [];

    for (let i = 0; i < size; i += 1) {
      const type = p_values[i * 3 + 1];
      if (type == 31) {
        // string
        const value = decoder.decode(
          this.memory.buffer.slice(
            p_values[i * 3 + 0],
            p_values[i * 3 + 0] + p_values[i * 3 + 2]
          )
        );
        values[i] = value;
      } else {
        throw new Error("not a string");
      }
    }

    this.deallocate(ptr, size * 3 * 4);
    return { data: values };
  }
}

const memory = new Memory(instance.exports);

const { init, apply, register } = instance.exports as Record<
  string,
  CallableFunction
>;
init();

context.memory;

const f = `lambda x: print(open("/app/plugin.py", "r"))`;

const op = memory.decode(
  register(...memory.encode("foo", "lambda x: x['a'] + 1"))
);
if (op.error) {
  console.log("python error", op.error);
}

for (let i = 0; i < 3; i += 1) {
  console.time("foo");
  const ret = memory.decode(
    apply(...memory.encode("foo", JSON.stringify({ a: 1 })))
  );
  if (ret.data) {
    console.log(ret.data.map((x) => JSON.parse(x as string)));
  } else {
    console.log("python error", ret.error);
  }
  console.timeEnd("foo");
}

//const add = instance.exports.add as CallableFunction;
//console.log(add(1, 2));
console.log("end");
