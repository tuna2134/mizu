use pyo3::prelude::*;
mod parser;

/// Mizu's core
#[pymodule]
fn mizu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<parser::Markdown>()?;
    Ok(())
}
