use wasmedge_bindgen_macro::wasmedge_bindgen;
use crate::memory::host_result;
use crate::core::*;

pub mod host {
    #[link(wasm_import_module = "host")]
    extern "C" {
        pub fn callback(id: i32, value: i32) -> ();
    }
}

#[wasmedge_bindgen]
pub fn register_lambda(name: String, code: String) -> Result<String, String> {
    lambda::register(name, code)
}

#[wasmedge_bindgen]
pub fn unregister_lambda(name: String) -> Result<String, String> {
    lambda::unregister(name)
}

#[wasmedge_bindgen]
pub fn apply_lambda(id: i32, name: String, args: String) -> u8 {
    let run = lambda::apply(name, args);
    match run {
        Ok(res) => {
            let ptr = host_result(true, res);
            unsafe {
                host::callback(id, ptr);
            };
            0
        }
        Err(e) => {
            let ptr = host_result(false, e);
            unsafe {
                host::callback(id, ptr);
            };
            1
        }
    }
}

#[wasmedge_bindgen]
pub fn register_def(name: String, code: String) -> Result<String, String> {
    defun::register(name, code)
}

#[wasmedge_bindgen]
pub fn unregister_def(name: String) -> Result<String, String> {
    defun::unregister(name)
}

#[wasmedge_bindgen]
pub fn apply_def(id: i32, name: String, args: String) -> u8 {
    let run = defun::apply(name, args);
    match run {
        Ok(res) => {
            let ptr = host_result(true, res);
            unsafe {
                host::callback(id, ptr);
            };
            0
        }
        Err(e) => {
            let ptr = host_result(false, e);
            unsafe {
                host::callback(id, ptr);
            };
            1
        }
    }
}

#[wasmedge_bindgen]
pub fn register_module(name: String, code: String) -> Result<String, String> {
    module::register(name, code)
}

#[wasmedge_bindgen]
pub fn unregister_module(name: String) -> Result<String, String> {
    module::unregister(name)
}