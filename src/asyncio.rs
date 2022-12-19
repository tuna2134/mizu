use pyo3::prelude::*;

pub fn set_result(py: Python, loop_: PyObject, future: PyObject, result: String) -> PyResult<()> {
    loop_.call_method1(
        py,
        "call_soon_threadsafe",
        (future.getattr(py, "set_result")?, result),
    )?;
    Ok(())
}

pub fn create_future(py: Python, loop_: PyObject) -> PyResult<PyObject> {
    let future = loop_.call_method0(py, "create_future")?;
    Ok(future)
}
