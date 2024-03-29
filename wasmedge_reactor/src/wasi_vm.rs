
use std::path::PathBuf;
use anyhow::bail;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, params, Caller, ImportObjectBuilder, Module, VmBuilder, WasmValue, Vm, NeverType, ImportObject,
};
use once_cell::sync::OnceCell;

#[host_function]
pub fn callback(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    // println!("[host] callback");
    Ok(vec![])
}

static IMPORTS: OnceCell<ImportObject<NeverType>> = OnceCell::new();

pub fn get_or_init_imports() -> anyhow::Result<&'static ImportObject<NeverType>> {
    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), (), NeverType>("callback", callback, None)?
        .build::<NeverType>("host", None)?;
    Ok(IMPORTS.get_or_init(|| import))
}


pub fn init_reactor_vm(
    inp_preopens: Vec<String>,
    pythonlib_path: PathBuf,
    wasi_mod_path: PathBuf
) -> anyhow::Result<Vm> {
    // start config
    let common_options = CommonConfigOptions::default().threads(true);
    let host_options = HostRegistrationConfigOptions::default().wasi(true);
    let config = ConfigBuilder::new(common_options)
        .with_host_registration_config(host_options)
        .build()?;
    // end config

    // [!] module order matters
    let mut vm = VmBuilder::new()
        .with_config(config)
        .build()?;

    // FIXME:
    // https://github.com/WasmEdge/WasmEdge/issues/3085
    // locally scoped import ref that uses the host function
    // makes some bindings call segfault with exit code 11
    // Note: in version 0.8.1, register_import_module
    // only required my_import vs &my_import (current)

    // Current solution: make a global ref
    vm.register_import_module(get_or_init_imports()?)?;

    // load wasm module
    let module = Module::from_file(None, wasi_mod_path)?;
    let mut vm = vm.register_module(None, module)?;

    let wasi_module = vm.wasi_module_mut().unwrap();

    // prepare preopens
    let mut preopens = vec![
        format!("/usr/local/lib:{}:readonly", pythonlib_path.display()),
    ];
    preopens.extend(inp_preopens);
    let preopens = preopens.iter().map(|s| s.as_ref()).collect();
    wasi_module.initialize(None, None, Some(preopens));

    let exit_code = wasi_module.exit_code();
    if exit_code != 0 {
        bail!(
            "wasi_module.initialize failed and returned exit code: {:?}",
            exit_code
        )
    }

    // if wasi-vfs is not used, initialize the reactor as not done automatically
    let _init = vm.run_func(None, "_initialize", params!())?;
    Ok(vm)
}