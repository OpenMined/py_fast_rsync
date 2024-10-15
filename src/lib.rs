use fast_rsync::{Signature, SignatureOptions};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
fn diff(py: Python, signature_bytes: &[u8], data: &[u8]) -> PyResult<Py<PyBytes>> {
    let signature = Signature::deserialize(signature_bytes.into()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Signature error: {}", e))
    })?;
    let indexed_signature = signature.index();

    let mut out = Vec::with_capacity(data.len()); // Pre-allocate memory for the output
    fast_rsync::diff(&indexed_signature, data, &mut out).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Diff error: {:?}", e))
    })?;

    // Convert Vec<u8> to PyBytes and return
    Ok(PyBytes::new_bound(py, &out).into())
}

#[pyfunction]
fn apply(py: Python, base: &[u8], delta: &[u8]) -> PyResult<Py<PyBytes>> {
    // Apply the delta to the data
    let mut out = Vec::with_capacity(base.len() + delta.len()); // Pre-allocate memory for the output
    fast_rsync::apply(base, delta, &mut out).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Apply error: {:?}", e))
    })?;

    // Convert Vec<u8> to PyBytes and return
    Ok(PyBytes::new_bound(py, &out).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_fast_rsync(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(diff, m)?)?;
    m.add_function(wrap_pyfunction!(apply, m)?)?;

    let signature_submodule = PyModule::new_bound(py, "signature")?;
    signature_submodule.add_function(wrap_pyfunction!(calculate, m)?)?;
    m.add_submodule(&signature_submodule)?;
    Ok(())
}

// Signature module

#[pyfunction]
#[pyo3(signature = (data, block_size=4096, crypto_hash_size=8))]
fn calculate(
    py: Python,
    data: &[u8],
    block_size: u32,
    crypto_hash_size: u32,
) -> PyResult<Py<PyBytes>> {
    let signature = Signature::calculate(
        data,
        SignatureOptions {
            block_size,
            crypto_hash_size,
        },
    );

    Ok(PyBytes::new_bound(py, &signature.into_serialized()).into())
}
