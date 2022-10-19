use pyo3::prelude::*;
mod parser;

/// A Python module implemented in Rust.
#[pymodule]
fn mizu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parser::parse, m)?)?;
    Ok(())
}
