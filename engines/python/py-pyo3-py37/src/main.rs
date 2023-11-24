#[cfg(Py_3_7)]
use pyo3::{
    prelude::*,
    types::{PyModule},
};

fn main() -> PyResult<()> {

    Python::with_gil(|py| {

        let activators = PyModule::from_code(
            py,
            include_str!("../../../../samples/sample.py"),
            "filename",
            "module_name"
        );

        let result: String = activators.unwrap().getattr("result").unwrap().to_string();
        println!("The answer is: {}", result);

    });

    Ok(())
}