use pyo3::prelude::*;
mod core;

/// Mizu's core
#[pymodule]
fn mizu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::Mizu>()?;
    Ok(())
}
