// https://stackoverflow.com/a/65980693/3293227
// https://github.com/rust-lang/rust/pull/79997#issuecomment-759856446

#![cfg_attr(feature = "wasm", no_main)]

#[allow(unused_imports)]
use pyo3::prelude::*;

#[allow(unused_imports)]
#[cfg(feature = "wasm")]
use python_wasi_reactor::bindings::*;

// #[cfg(feature = "wasm")]
use python_wasi_reactor::core::rustpy::*;

use wlr_libpy::py_main::py_main;

#[cfg(feature = "wasm")]
#[no_mangle]
pub extern "C" fn init_python() {
    println!("[guest] Python init");
    // fix: python cannot be found on wasmedge
    // std::env::set_var("PYTHONHOME", "/usr/local");
    // std::env::set_var("PYTHONPATH", "/app");
    // std::env::set_var("PYTHONDONTWRITEBYTECODE", "1");
    pyo3::append_to_inittab!(reactor);
    pyo3::prepare_freethreaded_python();
    py_main(std::env::args().collect());
    // Python::with_gil(|py| {
    //     let _module = py.import("plugin")?;
    //     Ok::<(), PyErr>(())
    // })
    // .unwrap();
}

#[cfg(not(feature = "wasm"))]
#[allow(unused_imports)]
fn main() {
    pyo3::append_to_inittab!(reactor);
    pyo3::prepare_freethreaded_python();
    py_main(std::env::args().collect());
    println!("native");
}
