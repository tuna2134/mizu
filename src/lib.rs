use pyo3::prelude::*;
mod asyncio;
mod core;

/// Mizu's core
#[pymodule]
fn mizu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::Mizu>()?;
    Ok(())
}
