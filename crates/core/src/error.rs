/// Typed error types for LaTeX parsing failures.
///
/// Provides structured error information including position and descriptive
/// message for user-friendly error reporting.
use thiserror::Error;

/// Error type returned by parsing operations.
#[derive(Error, Debug)]
pub enum ParseError {
    /// Syntax error encountered during parsing.
    #[error("Parse error at position {position}: {message}")]
    SyntaxError {
        /// Byte position in the input where the error occurred.
        position: usize,
        /// Human-readable description of the error.
        message: String,
    },
    /// Invalid expression that could not be converted to a valid AST node.
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
}
