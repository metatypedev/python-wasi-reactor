import { getAssemblyInstance } from "./common.ts";
const { instance, memory } = await getAssemblyInstance();

const { init_python, apply_lambda, register_lambda } = instance.exports as Record<
  string,
  CallableFunction
>;
init_python();

const op = memory.decode(
  register_lambda(...memory.encode("foo", "lambda x, y:  x['a'] + str(y)"))
);

if (op.error) {
  console.log("python error", op.error);
  Deno.exit(1);
}

for (let i = 0; i < 3; i += 1) {
  console.time("foo");
  const args = [{ a: `hello` }, i];
  const ret = memory.decode(
    apply_lambda(...memory.encode(i, "foo", JSON.stringify(args)))
  );
  if (ret.data) {
    console.log("exit status", i, ":", ret.data[0]);
  } else {
    console.log("python error", ret.error);
  }
  console.timeEnd("foo");
}
