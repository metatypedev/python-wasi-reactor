use pyo3::prelude::*;

#[pyfunction]
pub fn reverse(str: String) -> PyResult<String> {
    println!("reverse: {}", str);
    Ok(str.chars().rev().collect::<String>())
}

#[pymodule]
pub fn reactor(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reverse, m)?)?;
    Ok(())
}