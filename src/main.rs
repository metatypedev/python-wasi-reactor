// https://stackoverflow.com/a/65980693/3293227
// https://github.com/rust-lang/rust/pull/79997#issuecomment-759856446
#![no_main]

use pyo3::prelude::*;
use pyo3::types::PyDict;

use wasmedge_bindgen_macro::wasmedge_bindgen;
mod memory;

use memory::host_result;

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
pub fn register(name: String, code: String) -> Result<String, String> {
    Python::with_gil(|py| {
        let module = py.import("plugin")?;
        let f = py.eval(&code, None, None)?;
        let ret = module
            .getattr("fs")?
            .downcast::<PyDict>()?
            .set_item(name.clone(), f)?;
        Ok::<String, PyErr>(name)
    })
    .map_err(|e| e.to_string())
}

#[wasmedge_bindgen]
pub fn unregister(name: String) -> Result<String, String> {
    Python::with_gil(|py| {
        let module = py.import("plugin")?;
        let ret = module
            .getattr("fs")?
            .downcast::<PyDict>()?
            .del_item(name.clone())?;
        Ok::<String, PyErr>(name)
    })
    .map_err(|e| e.to_string())
}

#[wasmedge_bindgen]
pub fn apply(id: i32, name: String, args: String) -> u8 {
    let run = serde_json::from_str::<serde_json::Value>(&args)
        .map_err(|e| e.to_string())
        .and_then(|pyargs| {
            Python::with_gil(|py| {
                let module = py.import("plugin")?;
                let native = pythonize::pythonize(py, &pyargs)?;
                let pyret = module.getattr("fs")?.get_item(name)?.call1((native,))?;
                let json: serde_json::Value = pythonize::depythonize(pyret)?;
                Ok::<serde_json::Value, PyErr>(json)
            })
            .map_err(|e| e.to_string())
        })
        .and_then(|pyret| Ok(pyret.to_string()));
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
