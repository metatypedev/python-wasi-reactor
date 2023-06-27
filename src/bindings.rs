use wasmedge_bindgen_macro::wasmedge_bindgen;
use crate::memory::host_result;
use crate::core::*;

pub mod host {
    #[link(wasm_import_module = "host")]
    extern "C" {
        pub fn callback(id: i32, value: i32) -> ();
    }
}

/// Set id to -1 if != apply
pub fn return_and_host_output(id: i32, out: Result<String, String>) -> String {
    let ret = match out {
        Ok(res) => {
            if id > 0 {
                let ptr = host_result(true, res.to_owned());
                unsafe {
                    host::callback(id, ptr);
                };
            }
            common::RetValue { value: res, error: false }
        }
        Err(e) => {
            if id > 0 {
                let ptr = host_result(false, e.to_owned());
                unsafe {
                    host::callback(id, ptr);
                };
            }
            common::RetValue { value: e, error: true }
        }
    };
    serde_json::to_string(&ret).unwrap()
}

#[wasmedge_bindgen]
pub fn identity(value: String) -> String {
    return_and_host_output(-1, Ok(value))
}

#[wasmedge_bindgen]
pub fn register_lambda(name: String, code: String) -> String {
    return_and_host_output(-1, lambda::register(name, code))
}

#[wasmedge_bindgen]
pub fn unregister_lambda(name: String) -> String {
    return_and_host_output(-1, lambda::unregister(name))
}

#[wasmedge_bindgen]
pub fn apply_lambda(id: i32, name: String, args: String) -> String {
    return_and_host_output(id, lambda::apply(name, args))
}

#[wasmedge_bindgen]
pub fn register_def(name: String, code: String) -> String {
    return_and_host_output(-1, defun::register(name, code))
}

#[wasmedge_bindgen]
pub fn unregister_def(name: String) -> String {
    return_and_host_output(-1, defun::unregister(name))
}

#[wasmedge_bindgen]
pub fn apply_def(id: i32, name: String, args: String) -> String {
    return_and_host_output(id, defun::apply(name, args))
}

#[wasmedge_bindgen]
pub fn register_module(name: String, code: String) -> String {
    return_and_host_output(-1, module::register(name, code))
}

#[wasmedge_bindgen]
pub fn unregister_module(name: String) -> String {
    return_and_host_output(-1, module::unregister(name))
}