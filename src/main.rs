// https://stackoverflow.com/a/65980693/3293227
// https://github.com/rust-lang/rust/pull/79997#issuecomment-759856446
#![no_main]

use pyo3::prelude::*;

use wasmedge_bindgen_macro::wasmedge_bindgen;

use python_wasi_reactor::memory::host_result;

pub mod host {
    #[link(wasm_import_module = "host")]
    extern "C" {
        pub fn callback(id: i32, value: i32) -> ();
    }
}

#[pyfunction]
fn reverse(str: String) -> PyResult<String> {
    println!("reverse: {}", str);
    Ok(str.chars().rev().collect::<String>())
}

#[pymodule]
fn reactor(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reverse, m)?)?;
    Ok(())
}

#[wasmedge_bindgen]
pub fn register_lambda(name: String, code: String) -> Result<String, String> {
    python_wasi_reactor::lambda::register(name, code)
}

#[wasmedge_bindgen]
pub fn unregister_lambda(name: String) -> Result<String, String> {
    python_wasi_reactor::lambda::unregister(name)
}

#[wasmedge_bindgen]
pub fn apply_lambda(id: i32, name: String, args: String) -> u8 {
    let run = python_wasi_reactor::lambda::apply(name, args);
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
    python_wasi_reactor::defun::register(name, code)
}

#[wasmedge_bindgen]
pub fn unregister_def(name: String) -> Result<String, String> {
    python_wasi_reactor::defun::unregister(name)
}

#[wasmedge_bindgen]
pub fn apply_def(id: i32, name: String, args: String) -> u8 {
    let run = python_wasi_reactor::defun::apply(name, args);
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
    python_wasi_reactor::module::register(name, code)
}

#[wasmedge_bindgen]
pub fn unregister_module(name: String) -> Result<String, String> {
    python_wasi_reactor::module::unregister(name)
}

#[no_mangle]
pub extern "C" fn init() {
    std::env::set_var("PYTHONPATH", "/app");
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");

    pyo3::append_to_inittab!(reactor);
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let _module = py.import("plugin")?;
        Ok::<(), PyErr>(())
    })
    .unwrap();
}
