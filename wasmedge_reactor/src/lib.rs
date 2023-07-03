mod wasi_vm;

use std::path::PathBuf;

use deno_bindgen::deno_bindgen;
use wasmedge_sdk_bindgen::{Bindgen, Param};

#[deno_bindgen]
struct WasiReactorInp {
    callee: String,
    args: Vec<String>
}

#[deno_bindgen]
enum WasiReactorOut {
    Ok { res: String },
    Err { message: String }
}

#[deno_bindgen]
struct WasiReactorConfig {
    pylib_path: String,
    wasi_mod_path: String,
    preopens: Vec<String>,
}

#[deno_bindgen]
fn greet(name: &str) {
  println!("Hello, {}!", name);
}

#[deno_bindgen]
fn run_wasi_func(
    input: WasiReactorInp, 
    config: WasiReactorConfig
) -> WasiReactorOut {
    let vm = wasi_vm::init_reactor_vm(
        config.preopens, 
        PathBuf::from(config.pylib_path), 
        PathBuf::from(config.wasi_mod_path)
    );

    if let Err(e) = vm {
        return WasiReactorOut::Err { message: e.to_string() };
    }

    let vm = vm.unwrap();

    let mut bg = Bindgen::new(vm);

    let args = input
        .args
        .iter()
        .map(|v| Param::String(v))
        .collect();
    match bg.run_wasm(input.callee, args) {
        Ok(ret) => {
            let ret = ret.unwrap().pop().unwrap().downcast::<String>().unwrap();
            WasiReactorOut::Ok { res: ret.as_ref().to_owned() }
        },
        Err(e) => WasiReactorOut::Err { message: e.to_string() }
    }
}