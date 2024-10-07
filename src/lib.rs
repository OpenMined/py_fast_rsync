use pyo3::prelude::*;
use fast_rsync::{Signature, SignatureOptions};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pyfunction]
fn diff(data: &[u8], new_data: &[u8]) -> PyResult<Vec<u8>> {
    // Calculate the signature and store it in a variable
    let calculated_signature = Signature::calculate(
        data,
        SignatureOptions {
            block_size: 4096,
            crypto_hash_size: 8,
        },
    );

    // Index the signature
    let signature = calculated_signature.index();

    let mut out = Vec::new();
    fast_rsync::diff(&signature, new_data, &mut out).unwrap();
    // Return the delta as a byte vector
    Ok(out)
}

#[pyfunction]
fn apply(base: &[u8], delta: &[u8]) -> PyResult<Vec<u8>> {
    // Apply the delta to the data
    let mut out = Vec::new();
    fast_rsync::apply(base, delta, &mut out).unwrap();
    // Return the patched data as a byte vector
    Ok(out)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_fast_rsync(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(diff, m)?)?;
    m.add_function(wrap_pyfunction!(apply, m)?)?;
    Ok(())
}
