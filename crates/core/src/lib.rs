pub mod ast;
pub mod builder;
pub mod error;
pub mod latex_parser;
pub mod translator;

pub use error::ParseError;

use builder::parse_to_ast;
use translator::translate;

/// Parse a LaTeX math expression and return its Chinese reading.
///
/// # Errors
///
/// Returns `ParseError` if the input cannot be parsed or contains
/// invalid syntax.
pub fn parse_latex(latex: &str) -> Result<String, ParseError> {
    let ast = parse_to_ast(latex)?;
    Ok(translate(&ast))
}
