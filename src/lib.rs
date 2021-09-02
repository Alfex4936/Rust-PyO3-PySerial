extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::PyList;
use std::cmp::Ordering;
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
        let str = format!("I am {}!", self.name);
        Ok(str)
    }
}

#[pyfunction]
fn bubble(_py: Python, array: &PyList) -> PyResult<()> {
    let mut count = 0;
    let mut is_sorted = false;
    let n = array.len() - 1;

    while !is_sorted {
        is_sorted = true;
        for i in 0..(n - count) as isize {
            let first = array.get_item(i);
            let second = array.get_item(i + 1);
            match first.compare(second).unwrap() {
                Ordering::Greater => {
                    array.set_item(i, second)?;
                    array.set_item(i + 1, first)?;
                    is_sorted = false;
                }
                _ => {}
            }
        }

        count += 1;
    }

    Ok(())
}

#[pymodule]
fn rust_sort(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Human>()?;
    m.add_function(wrap_pyfunction!(bubble, m)?)?;

    Ok(())
}
