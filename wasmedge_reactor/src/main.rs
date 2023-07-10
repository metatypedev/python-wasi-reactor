mod wasi_vm;
mod wasmedge_sdk_bindgen;

use std::path::PathBuf;

use wasmedge_sdk::params;
use wasmedge_sdk_bindgen::{Bindgen, Param};

fn main() -> anyhow::Result<()> {
    let preopens = vec!["/app:./src/python:readonly".to_owned()];
    let pylib = PathBuf::from("./vendor/libpython/usr/local/lib");
    let wasi_mod = PathBuf::from("./build/python-wasi-reactor.wasm");
    let vm = wasi_vm::init_reactor_vm(preopens, pylib, wasi_mod)?;

    println!("\n-----------------");
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
