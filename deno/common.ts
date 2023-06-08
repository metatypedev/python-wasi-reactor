import Context from "https://deno.land/std@0.178.0/wasi/snapshot_preview1.ts";
import { Memory } from "https://raw.githubusercontent.com/metatypedev/metatype/main/typegate/src/runtimes/python_wasi/memory.ts";


interface HostCallback {
    (id: number, ptr: number): void
}

export async function getAssemblyInstance(customCallback?: HostCallback) {
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
        callback: customCallback ?? function(id: number, ptr: number) {
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

    return {instance, memory};
}