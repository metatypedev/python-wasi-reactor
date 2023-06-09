// https://stackoverflow.com/a/65980693/3293227
// https://github.com/rust-lang/rust/pull/79997#issuecomment-759856446

#![cfg_attr(feature = "wasm", no_main)]

#[allow(unused_imports)]
use pyo3::prelude::*;

#[allow(unused_imports)]
#[cfg(feature = "wasm")]
use python_wasi_reactor::export::*;

#[cfg(feature = "wasm")]
use python_wasi_reactor::core::rustpy::*;

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
