use anyhow::Result;
use std::time::Instant;
use wasmedge_sdk::error::HostFuncError;
use wasmedge_sdk::{host_function, Caller};
use wasmedge_sys::{Config, ImportObject, Store, Vm, WasmValue};

use wasmedge_sys::{AsImport, FuncType};
use wasmedge_sys::{Function, WasiModule};

#[host_function]
pub fn sock_accept(
    _caller: Caller,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<()> {
    // https://github.com/second-state/wasmedge-rustsdk-examples

    let mut config = Config::create()?;
    config.wasi(true);

    let mut store = Store::create()?;
    let mut vm = Vm::create(Some(config), Some(&mut store))?;
    vm.register_wasm_from_file("extern", "./target/wasm32-wasi/release/wrapper.wasm")?;

    let mut wasi_import = WasiModule::create(
        None,
        Some(vec!["PYTHONPATH=/app", "PYTHONDONTWRITEBYTECODE=1"]),
        Some(vec!["/app:./python", "/usr:./wrapper/deps/usr"]),
    )?;
    wasi_import.add_func(
        "sock_accept",
        Function::create(&FuncType::create([], [])?, Box::new(sock_accept), 0)?,
    );
    vm.register_wasm_from_import(ImportObject::Wasi(wasi_import))?;

    println!("run");
    let now = Instant::now();
    vm.run_registered_function("extern", "_initialize", [])?;

    println!("{:?}: {:?}", now.elapsed(), 2);
    Ok(())
}
