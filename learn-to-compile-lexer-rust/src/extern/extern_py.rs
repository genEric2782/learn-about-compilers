// use std::string;
use std::ffi::{CStr, CString};
use pyo3::prelude::*;
use pyo3::types::PyModule;
// use serde::Serialize;

pub fn evaluate_ast_with_python(node: &str) -> PyResult<i64> {

    let python_code = include_str!("../../../learn-to-compile-evaluator-python/src/main.py");
    let python_code_c = CString::new(python_code).expect("CString conversion failed");

    Python::with_gil(|py| {
        // Load your Python module from the source string
        let module = PyModule::from_code(py, &python_code_c, c"main.py", c"evaluator")?;

        // Call evaluate_from_json(ast_json) -> float
        let result: i64 = module
            .getattr("evaluate_ast_from_json")?
            .call1((node,))?
            .extract()?;

        Ok(result)
    })
}