use pyo3::prelude::*;
use fast_rsync::{Signature, SignatureOptions};
use pyo3::types::PyBytes;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}



#[pyfunction]
fn diff(py: Python, data: &[u8], new_data: &[u8]) -> PyResult<Py<PyBytes>> {
    // Calculate the signature and store it in a variable
    let calculated_signature = Signature::calculate(
        data,
        SignatureOptions {
            block_size: 4096, // Adjust this based on your data characteristics
            crypto_hash_size: 8,
        },
    );

    // Index the signature
    let signature = calculated_signature.index();

    let mut out = Vec::with_capacity(new_data.len()); // Pre-allocate memory for the output
    fast_rsync::diff(&signature, new_data, &mut out)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Diff error: {:?}", e)))?;

    // Convert Vec<u8> to PyBytes and return
    Ok(PyBytes::new_bound(py, &out).into())
}

#[pyfunction]
fn apply(py: Python, base: &[u8], delta: &[u8]) -> PyResult<Py<PyBytes>> {
    // Apply the delta to the data
    let mut out = Vec::with_capacity(base.len() + delta.len()); // Pre-allocate memory for the output
    fast_rsync::apply(base, delta, &mut out)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Apply error: {:?}", e)))?;

    // Convert Vec<u8> to PyBytes and return
    Ok(PyBytes::new_bound(py, &out).into())
}


/// A Python module implemented in Rust.
#[pymodule]
fn py_fast_rsync(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(diff, m)?)?;
    m.add_function(wrap_pyfunction!(apply, m)?)?;
    Ok(())
}
