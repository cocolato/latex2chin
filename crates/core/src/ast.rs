//! AST types for representing parsed LaTeX mathematical expressions.
//!
//! Each expression variant corresponds to a distinct LaTeX syntax construct.
//! All types derive Debug, Clone, and PartialEq for testing and transformation.

/// Top-level expression node representing any LaTeX math construct.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Numeric literal: `3.14`, `42`
    Number(f64),
    /// Single-letter variable: `x`, `A`
    Identifier(String),
    /// Greek letter constant: `\alpha`, `\pi`, etc.
    Greek(GreekLetter),
    /// Arithmetic binary operation: `a + b`, `x * y`
    BinaryOp(BinaryOpKind, Box<Expr>, Box<Expr>),
    /// Comparison binary operation: `a = b`, `x < y`
    ComparisonOp(ComparisonOpKind, Box<Expr>, Box<Expr>),
    /// Fraction: `\frac{num}{den}`
    Frac(Box<Expr>, Box<Expr>),
    /// Square root: `\sqrt{x}`
    Sqrt(Box<Expr>),
    /// Nth root: `\sqrt[n]{x}` (first = degree, second = radicand)
    SqrtN(Box<Expr>, Box<Expr>),
    /// Superscript: `x^{n}`, `x^2`
    Superscript(Box<Expr>, Box<Expr>),
    /// Subscript: `a_{n}`, `a_1`
    Subscript(Box<Expr>, Box<Expr>),
    /// Mathematical function application: `\sin x`, `\log{x}`
    Function(MathFunction, Box<Expr>),
    /// Parenthesized group: `(expr)`
    Group(Box<Expr>),
    /// Plus-minus: `\pm x`
    Pm(Box<Expr>),
    /// Minus-plus: `\mp x`
    Mp(Box<Expr>),
    /// Degree suffix: `90\degree`
    Degree(Box<Expr>),
    /// Percent suffix: `50%`
    Percent(Box<Expr>),
    /// Sign prefix: `+x` or `-x`
    Sign(SignKind, Box<Expr>),
    /// Limit: `\lim_{x \to 0} body` (variable, target, body)
    Limit(Box<Expr>, Box<Expr>, Box<Expr>),
    /// Summation: `\sum_{i=1}^{n} body` (variable, lower, upper, body)
    Sum(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    /// Product: `\prod_{i=1}^{n} body` (variable, lower, upper, body)
    Product(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    /// Integral: `\int_{a}^{b} body` (lower, upper, body)
    Integral(Box<Expr>, Box<Expr>, Box<Expr>),
    /// Set theory binary operation: `a \in B`, `A \cup B`
    SetOp(SetOpKind, Box<Expr>, Box<Expr>),
    /// Logic binary operation: `A \Rightarrow B`, `A \iff B`
    LogicOp(LogicOpKind, Box<Expr>, Box<Expr>),
    /// Geometry symbol: `\triangle`, `\angle`
    Geometry(GeometrySymbol),
    /// Empty set constant: `\emptyset`, `\varnothing`
    Emptyset,
}

/// Arithmetic operator kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

/// Comparison operator kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOpKind {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Approx,
    Napprox,
    /// Arrow: `x \to 0`
    To,
    /// Parallel: `A \parallel B`
    Parallel,
    /// Perpendicular: `A \perp B`
    Perpendicular,
    /// Congruent: `A \cong B`
    Congruent,
    /// Similar: `A \sim B`
    Similar,
}

/// Sign prefix kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum SignKind {
    Positive,
    Negative,
}

/// Common Greek letters used in mathematical notation.
#[derive(Debug, Clone, PartialEq)]
pub enum GreekLetter {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Theta,
    Lambda,
    Mu,
    Sigma,
    Pi,
    Phi,
    Omega,
}

/// Mathematical function names.
#[derive(Debug, Clone, PartialEq)]
pub enum MathFunction {
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    Log,
    Ln,
    Lg,
}

/// Set theory operator kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum SetOpKind {
    In,
    NotIn,
    Cup,
    Cap,
    Subset,
    Superset,
    Emptyset,
}

/// Logic operator kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicOpKind {
    Forall,
    Exists,
    Implies,
    Iff,
}

/// Geometry symbol kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum GeometrySymbol {
    Triangle,
    Angle,
    Parallel,
    Perpendicular,
    Congruent,
    Similar,
}
