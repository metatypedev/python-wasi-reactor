// https://stackoverflow.com/a/65980693/3293227
// https://github.com/rust-lang/rust/pull/79997#issuecomment-759856446
#![no_main]

use pyo3::prelude::*;
use pyo3::types::PyDict;
use wasmedge_bindgen_macro::wasmedge_bindgen;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    println!("testt");
    Ok((a + b).to_string())
}

#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn allocate(size: i32) -> *const u8 {
    let buffer = Vec::with_capacity(size as usize);
    let buffer = std::mem::ManuallyDrop::new(buffer);
    buffer.as_ptr() as *const u8
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(pointer: *mut u8, size: i32) {
    drop(Vec::from_raw_parts(pointer, size as usize, size as usize));
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
pub fn apply(name: String, json: String) -> Result<String, String> {
    Python::with_gil(|py| {
        let module = py.import("plugin")?;
        let args = serde_json::from_str::<serde_json::Value>(&json).unwrap();
        let native = pythonize::pythonize(py, &args)?;
        let ret = module.getattr("fs")?.get_item(name)?.call1((native,))?;
        let res: serde_json::Value = pythonize::depythonize(ret)?;
        Ok::<String, PyErr>(res.to_string())
    })
    .map_err(|e| e.to_string())
}

#[no_mangle]
pub extern "C" fn init() {
    std::env::set_var("PYTHONPATH", "/app");
    std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");

    pyo3::append_to_inittab!(string_sum);
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let _module = py.import("plugin")?;
        Ok::<(), PyErr>(())
    })
    .unwrap();
}
