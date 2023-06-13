use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, Module,
    VmBuilder, WasmValue, config::{HostRegistrationConfigOptions, ConfigBuilder, CommonConfigOptions},
};
use wasmedge_sdk_bindgen::{Bindgen, Param};
 
#[host_function]
pub fn callback(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world from host callback!");
    for arg in _args {
        println!(" arg: {:?}", arg);
    }
    Ok(vec![])
}
 
#[cfg_attr(test, test)]
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
    
    // loads wasm module
    let module = Module::from_file(None, "build/python-wasi-reactor.wasm")?;
 
    // create an import module
    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), ()>("callback", callback)?
        .build("host")?;

    // [!] module order matters
    let vm = VmBuilder::new()
        .with_config(config)
        .build()?
        .register_import_module(import)?
        .register_module(None, module)?;

    let args = vec![
        WasmValue::from_i32(1234), // id
        WasmValue::from_i32(5678), // ptr
    ];
    vm.run_func(Some("host"), "callback", args)?;
    vm.run_func(None, "init", params!())?;

    let mut bg = Bindgen::new(vm);

    // basic test
    match bg.run_wasm("identity", vec![Param::String("hello world")]) {
        Ok(rv) => {
            println!(
                "Run bindgen -- identity: {:?}",
                rv.unwrap().pop().unwrap().downcast::<String>().unwrap()
            );
        }
        Err(e) => {
            println!("Run bindgen -- identity FAILED {:?}", e);
        }
    }
    Ok(())
}