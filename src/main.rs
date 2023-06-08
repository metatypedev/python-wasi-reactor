// https://stackoverflow.com/a/65980693/3293227
// https://github.com/rust-lang/rust/pull/79997#issuecomment-759856446


#![no_main]

use pyo3::prelude::*;

#[allow(unused_imports)]
#[cfg(feature = "wasm")]
use python_wasi_reactor::export::*;

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

#[cfg(feature = "wasm")]
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

#[cfg(not(feature = "wasm"))]
#[allow(unused_imports)]
fn main() {
    println!("native");
}
