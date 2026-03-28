use latex2chin_core::parse_latex as core_parse_latex;
use pyo3::prelude::*;

#[pyfunction]
pub fn parse_latex(latex: String) -> Result<String, PyErr> {
    core_parse_latex(&latex).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn latex2chin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_latex, m)?)?;
    Ok(())
}
