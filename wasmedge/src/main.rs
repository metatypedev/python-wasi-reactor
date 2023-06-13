use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, Module,
    VmBuilder, WasmValue, config::{HostRegistrationConfigOptions, ConfigBuilder, CommonConfigOptions},
};
use wasmedge_sdk_bindgen::{Bindgen, Param};
 
#[host_function]
pub fn callback(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("[host] callback");
    for arg in _args {
        println!(" arg: {:?}", arg);
    }
    Ok(vec![])
}

pub fn bindgen_exec(bg: &mut Bindgen, callee: &str, args: Vec<Param>) {
    match bg.run_wasm(callee, args) {
        Ok(rv) => {
            println!(
                "Run bindgen -- {}: {:?}",
                callee,
                rv.unwrap().pop().unwrap().downcast::<String>().unwrap()
            );
        }
        Err(e) => {
            println!("Run bindgen -- {} FAILED {:?}", callee, e);
        }
    }
}

fn main() -> anyhow::Result<()> {
    // start config
    let common_options = CommonConfigOptions::default()
        .bulk_memory_operations(true)
        .multi_value(true)
        .mutable_globals(true)
        .non_trap_conversions(true)
        .reference_types(true)
        .sign_extension_operators(true)
        .simd(true);
    let host_options = HostRegistrationConfigOptions::default()
        .wasi(true);
    let config = ConfigBuilder::new(common_options)
        .with_host_registration_config(host_options)
        .build()
        .unwrap();
    // end config
    
    // load wasm module
    let module = Module::from_file(None, "build/python-wasi-reactor.wasm")?;
 
    // create an import module
    let imports = ImportObjectBuilder::new()
        .with_func::<(i32, i32), ()>("callback", callback)?
        .build("host")?;

    // [!] module order matters
    let vm = VmBuilder::new()
        .with_config(config)
        .build()?
        .register_import_module(imports)?
        .register_module(None, module)?;

    // let args = vec![
    //     WasmValue::from_i32(1234), // id
    //     WasmValue::from_i32(5678), // ptr
    // ];
    // vm.run_func(Some("host"), "callback", args)?;
    vm.run_func(None, "init", params!())?;

    let mut bg = Bindgen::new(vm);

    // basic test
    bindgen_exec(&mut bg, "identity", vec![Param::String("hello identity from guest")]);

    bindgen_exec(&mut bg, "register_lambda", vec![
        Param::String("say_hello"),
        Param::String("lambda name: f\"Hello {name}\"")
    ]);

    // bindgen_exec(&mut bg, "apply_lambda", vec![
    //     Param::String("say_hello"),
    //     Param::String("[\"Jake!\"]") // json array
    // ]);

    // match bg.run_wasm("register_lambda", args) {
    //     Ok(rv) => {
    //         println!(
    //             "Run bindgen -- register_lambda: {:?}",
    //             rv.unwrap().pop().unwrap().downcast::<String>().unwrap()
    //         );
    //     }
    //     Err(e) => {
    //         println!("Run bindgen -- register_lambda FAILED {:?}", e);
    //     }
    // }
    Ok(())
}