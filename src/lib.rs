use pyo3::{prelude::*, types::PyString};

/// Generates a nanoid string.
#[pyfunction]
fn generate(alphabet: Bound<PyString>, size: usize) -> PyResult<String> {
    // TODO: Implement the nanoid generation.
    Ok(alphabet.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_nanoid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    Ok(())
}
