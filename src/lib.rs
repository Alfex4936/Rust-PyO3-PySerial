extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::PyList;

#[pymodule]
fn rust_sort(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "bubble")]
    fn bubble(_py: Python, array: &PyList) {
        for val in array.iter() {
            println!("{}", val);
        }
    }
    Ok(())
}
