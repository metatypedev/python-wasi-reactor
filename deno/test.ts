import { assertEquals } from "https://deno.land/std@0.190.0/testing/asserts.ts";
import { getAssemblyInstance } from "./common.ts";


const global = { _ret: undefined } as Record<string, any>;
function customCallback(id: number, ptr: number) {
    global.ret = memory.decode(ptr);
}

const { instance, memory } = await getAssemblyInstance(customCallback);
const { 
    init_python, 
  register_lambda,                 

  apply_lambda,    
  register_def,

  apply_def,
  register_module,
} = instance.exports as Record<
    string,
    CallableFunction
  >;
init_python();

Deno.test("wasm bindings", async (t) => {
    await t.step("lambda function", () => {
        const name = "hello", args = [1234];
        const code = 'lambda x: f"hello{x}"';
        // register
        const op = memory.decode(
            register_lambda(...memory.encode(name, code))
        );
        if (op.error) {
            throw Error("python error "+ op.error);
        }
        // call
        const _ = memory.decode(
            apply_lambda(...memory.encode(0, name, JSON.stringify(args)))
        );
        
        assertEquals({data: ['"hello1234"']}, global.ret)
    });

    await t.step("def function", () => {
        const name = "hello", args = [1234];
        const code = 'def hello(x):\n\treturn f"hello{x}"';
        // register
        const op = memory.decode(
            register_def(...memory.encode(name, code))
        );
        if (op.error) {
            throw Error("python error "+ op.error);
        }
        // call
        const _ = memory.decode(
            apply_def(...memory.encode(0, name, JSON.stringify(args)))
        );

        assertEquals({data: ['"hello1234"']}, global.ret);
    });

    await t.step("module", () => {
        const mod = "module";
        const calls = ["A", "B"], args = <string[]>[];
        const code = [
            'def A():\n\treturn "A"', 
            'def B():\n\treturn "B"'
        ].join("\n\n");
        // register
        const op = memory.decode(
            register_module(...memory.encode(mod, code))
        );
        if (op.error) {
            throw Error("python error "+ op.error);
        }
        // call
        for (const name of calls) {
            const callee = `${mod}.${name}`;
            const _ = memory.decode(
                apply_def(...memory.encode(0, callee, JSON.stringify(args)))
            );
            assertEquals({data: [JSON.stringify(name)]}, global.ret);
        }
    });
});