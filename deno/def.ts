import { getAssemblyInstance } from "./common.ts";
const { instance, memory } = await getAssemblyInstance();

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
  {
    name: 'raiseErrorIdentity', 
    args: ["some error message for test"],
    code: `
def raiseErrorIdentity(x):
  raise Exception(str(x))
    `
  },
  {
    name: 'sum', 
    args: new Array(100).fill(1),
    code: `
def sum(*args):
  s = 0
  for n in args:
    s += n
  return s
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