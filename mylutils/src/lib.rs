use pyo3::prelude::*;
use std::fs;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_txt<'a>(file_path: &str) -> Vec<String> {
    let lines = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");
    // vec![lines.lines().collect()]
    lines.lines().map(str::to_string).collect()
}

/// A Python module implemented in Rust.
#[pymodule]
fn mylutils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_txt, m)?)?;
    Ok(())
}