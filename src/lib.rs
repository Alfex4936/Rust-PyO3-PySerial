extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass(subclass)]
pub struct Human {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    age: u8,
    #[pyo3(get)]
    salary: u32,
}

#[pymethods]
impl Human {
    #[new]
    fn new(name: String, age: u8, salary: u32) -> Self {
        Human { name, age, salary }
    }

    fn promote(&mut self, salary: u32) {
        self.salary = salary;
    }

    fn string(&self) -> PyResult<String> {
        let str = format!("I am {}!", self.name).to_string();
        Ok(str)
    }
}

#[pymodule]
fn rust_sort(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "bubble")]
    fn bubble(_py: Python, array: &PyList) {
        for val in array.iter() {
            println!("{}", val);
        }
    }
    m.add_class::<Human>()?;

    Ok(())
}
