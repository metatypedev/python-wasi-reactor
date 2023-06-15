import { getAssemblyInstance } from "./common.ts";
const { instance, memory } = await getAssemblyInstance();

// init, apply and register are all in main.rs
// works similarly to bindgen
const { 
  init, 
  register_module,
  apply_def,
} = instance.exports as Record<
  string,
  CallableFunction
>;
init();


const tests = [
  {
    module: 'parent',
    calls: [
      { name: 'A', args: []},
      { name: 'B', args: []},
    ],
    code: `
def A():
    return "A"

def B():
  return "say A"
`
  },

  {
    module: 'my_mod',
    calls: [
      { name: 'even', args: [456]},
      { name: 'odd', args: [-456]},
    ],
    code: `
def even(x):
  if x == 0:
    return True
  if x == -1:
    return False
  return odd(abs(x) - 1)

def odd(x):
  return even(abs(x) - 1)
`
  },
  {
    module: 'ext_folder_expose',
    calls: [
      { name: 'fn_that_calls_host', args: []},
    ],
    code: `
def fn_that_calls_host():
  import sys
  sys.path.insert(0, '/host_py') # import lookup, see commons
  import host # see preopens
  from nested_a.nested_b import fn_nested

  return [
    host.hello_host(),
    fn_nested(),
    f"version {sys.version}",
    # sys.path
  ]
`
  },
]

// register
for (const {module, code} of tests) {
  const op = memory.decode(
    register_module(...memory.encode(module, code))
  );
  if (op.error) {
    console.log("python error", op.error);
    Deno.exit(1);
  }
}

// call
let id = 1;
for (const {module, calls} of tests) {
  for (const {name, args} of calls) {
    const callee = `${module}.${name}`;
    console.log("\ncalling %s(%s) ...", callee, args.join(','));
    const ret = memory.decode(
      apply_def(...memory.encode(id++, callee, JSON.stringify(args)))
    );
    console.log(ret);
    if (ret.data ) {
      console.log("exit status", ret.data[0]);
    } else {
        console.log("python error", ret.error);
    }
  }
}