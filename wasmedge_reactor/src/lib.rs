mod wasi_vm;

use std::{path::PathBuf, sync::Arc};

use deno_bindgen::deno_bindgen;
use wasmedge_sdk::Vm;
use wasmedge_sdk_bindgen::{Bindgen, Param};
use once_cell::sync::OnceCell;
use arc_swap::{ArcSwap, Guard};

static GLOBAL_VM: OnceCell<ArcSwap<Vm>> = OnceCell::new();

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
    reset_vm: bool
}

fn get_global_vm(config: WasiReactorConfig) -> Result<Guard<'static, Arc<Vm>>, String> {
    if GLOBAL_VM.get().is_none() || config.reset_vm {
        let ret = wasi_vm::init_reactor_vm(
            config.preopens, 
            PathBuf::from(config.pylib_path), 
            PathBuf::from(config.wasi_mod_path)
        );
        if let Err(e) = ret {
            return Err(e.to_string());
        }
        if GLOBAL_VM.get().is_some() {
            GLOBAL_VM
                .get()
                .unwrap()
                .store(Arc::new(ret.unwrap()));
        } else {
            GLOBAL_VM
                .set(ArcSwap::from_pointee(ret.unwrap()))
                .unwrap();
        }
    }
    Ok(GLOBAL_VM.get().unwrap().load())
}

#[deno_bindgen]
fn run_wasi_func(
    input: WasiReactorInp, 
    config: WasiReactorConfig
) -> WasiReactorOut {

    let vm = get_global_vm(config);
    if let Err(e) = vm {
        return WasiReactorOut::Err { message: e.to_string() };
    }

    let vm = vm.as_deref().unwrap().as_ref();
    let mut bg = Bindgen::new(vm.to_owned());

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