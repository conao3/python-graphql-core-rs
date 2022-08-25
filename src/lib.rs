use graphql_parser;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn parse(query: &str) -> PyResult<String> {
    let ast_result = graphql_parser::parse_query::<&str>(query);
    if let Err(e) = ast_result {
        return Err(pyo3::exceptions::PyValueError::new_err(e.to_string()));
    }
    let ast = ast_result.unwrap();
    Ok(ast.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn graphql_core_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
