use pyo3::prelude::*;
mod parser;

/// Mizu's core
#[pymodule]
fn mizu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parser::parse, m)?)?;
    m.add_function(wrap_pyfunction!(parser::parse_ext, m)?)?;
    m.add_class::<parser::Markdown>()?;
    Ok(())
}
