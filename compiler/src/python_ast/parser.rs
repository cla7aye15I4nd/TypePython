use crate::error::{CompilerError, Result};
use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Parse Python source code into a Python AST using Python's ast.parse()
pub fn parse_python(source: &str) -> Result<Py<PyAny>> {
    Python::attach(|py| {
        // Import the ast module
        let ast_module = PyModule::import(py, "ast").unwrap();

        // Call ast.parse(source)
        let parsed = ast_module.call_method1("parse", (source,)).map_err(|e| {
            CompilerError::ParseError(format!("Failed to parse Python source: {}", e))
        })?;

        Ok(parsed.into())
    })
}
