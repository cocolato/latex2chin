use latex2chin_core::parse_latex as core_parse_latex;
use pyo3::prelude::*;

// Re-export for test compatibility
pub use latex2chin_core::latex_parser;
pub use latex2chin_core::parse_latex;

#[pyfunction]
pub fn parse_latex(latex: String) -> String {
    core_parse_latex(&latex)
}

#[pymodule]
fn latex2chin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_latex, m)?)?;
    Ok(())
}
