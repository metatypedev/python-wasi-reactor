use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, params, Caller, ImportObjectBuilder, Module, VmBuilder, WasmValue,
};
use wasmedge_sdk_bindgen::{Bindgen, Param};

#[host_function]
pub fn callback(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("[host] callback");
    if _args.len() != 2 {
        panic!("{} != 2 (required)", _args.len());
    }

    Ok(vec![])
}

fn main() -> anyhow::Result<()> {
    // start config
    let common_options = CommonConfigOptions::default().threads(true);
    let host_options = HostRegistrationConfigOptions::default().wasi(true);
    let config = ConfigBuilder::new(common_options)
        .with_host_registration_config(host_options)
        .build()?;
    // end config

    // load wasm module
    let module = Module::from_file(None, "build/python-wasi-reactor.wasm")?;

    // create an import module
    let imports = ImportObjectBuilder::new()
        .with_func::<(i32, i32), ()>("callback", callback)?
        .build("host")?;

    // [!] module order matters
    let mut vm = VmBuilder::new()
        .with_config(config)
        .build()?
        .register_import_module(imports)?
        .register_module(None, module)?;

    let wasi_module = vm.wasi_module_mut().unwrap();
    let preopens = vec![
        "/app:./src/python:readonly",
        "/usr/local/lib:./vendor/libpython/usr/local/lib:readonly",
    ];

    wasi_module.initialize(None, None, Some(preopens));
    println!("wasi_module: {:?}", wasi_module.exit_code());

    // if wasi-vfs is not used, initialize the reactor as not done automatically
    let init = vm.run_func(None, "_initialize", params!())?;
    println!("init: {:?}", init);

    println!("\n-----------------");
    // let args = vec![
    //     WasmValue::from_i32(1234), // id
    //     WasmValue::from_i32(5678), // ptr
    // ];
    // vm.run_func(Some("host"), "callback", args)?;
    vm.run_func(None, "init_python", params!())?;

    let mut bg = Bindgen::new(vm);

    // basic test
    bg.run_wasm("identity", vec![Param::String("hello identity from guest")])
        .and_then(|rv| {
            let ret = rv.unwrap().pop().unwrap().downcast::<String>().unwrap();
            println!("Run bindgen -- identity {:?}", ret);
            Ok(())
        })?;

    let args = vec![
        Param::String("say_hello"),
        Param::String("lambda name: f\"Hello {name}\"")
    ];
    bg.run_wasm("register_lambda", args)
        .and_then(|rv| {
            let ret = rv.unwrap().pop().unwrap().downcast::<String>().unwrap();
            println!("Run bindgen -- register_lambda {:?}", ret);
            Ok(())
        })?;


    let args = vec![
        Param::String("1"),
        Param::String("say_hello"),
        Param::String("[\"Jake\"]")
    ];
    bg.run_wasm("apply_lambda", args)
        .and_then(|rv| {
            let ret = rv.unwrap().pop().unwrap().downcast::<String>().unwrap();
            println!("Run bindgen -- apply_lambda {}", ret);
            Ok(())
        })?;
    Ok(())
}
