/// Pest parse tree to AST builder.
///
/// Converts pest `Pair<Rule>` nodes into typed `Expr` AST nodes.
/// Every grammar rule has a corresponding builder function that
/// produces the correct AST variant.
use crate::ast::*;
use crate::error::ParseError;
use crate::latex_parser::{LatexParser, Rule};
use pest::iterators::Pair;
use pest::Parser;

/// Parse a LaTeX string and return the corresponding AST expression.
///
/// # Errors
///
/// Returns a `ParseError` if parsing fails or if the parse tree
/// cannot be converted to a valid AST node.
pub fn parse_to_ast(input: &str) -> Result<Expr, ParseError> {
    let pairs = LatexParser::parse(Rule::input, input).map_err(|e| {
        let pos = match e.location {
            pest::error::InputLocation::Pos(p) => p,
            pest::error::InputLocation::Span((s, _)) => s,
        };
        ParseError::SyntaxError {
            position: pos,
            message: e.to_string(),
        }
    })?;
    let input_pair = pairs
        .into_iter()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "No input pair found".to_string(),
        })?;
    let expr_pair = input_pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::expr)
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "No expression found in input".to_string(),
        })?;
    build_expr(expr_pair)
}

// ---------------------------------------------------------------------------
// Expression builder (top-level rule)
// ---------------------------------------------------------------------------

/// Build an `Expr` from a pest `expr` pair.
///
/// The grammar rule is:
/// ```text
/// expr = { sign* ~ term ~ (binary_op ~ sign* ~ term)* }
/// ```
///
/// Binary operations are folded left-to-right to preserve
/// left-to-right associativity.
fn build_expr(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner: std::iter::Peekable<_> = pair.into_inner().peekable();

    // Collect leading prefix signs
    let mut leading_signs = Vec::new();
    while let Some(p) = inner.peek() {
        if p.as_rule() == Rule::sign {
            leading_signs.push(inner.next().unwrap());
        } else {
            break;
        }
    }

    // First term (mandatory)
    let first_term_pair = inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected term in expression".to_string(),
    })?;
    let mut result = build_term(first_term_pair)?;

    // Apply leading signs in reverse order (innermost first)
    for sign_pair in leading_signs.into_iter().rev() {
        result = apply_sign(sign_pair, result)?;
    }

    // Fold remaining (binary_op, signs, term) groups left-to-right
    loop {
        // Expect a binary operator
        let op_pair = match inner.peek() {
            Some(p) if is_binary_op_rule(p.as_rule()) => inner.next().unwrap(),
            _ => break,
        };

        // Collect signs between operator and the next term
        let mut trailing_signs = Vec::new();
        while let Some(p) = inner.peek() {
            if p.as_rule() == Rule::sign {
                trailing_signs.push(inner.next().unwrap());
            } else {
                break;
            }
        }

        // Next term (mandatory after an operator)
        let next_term_pair = inner.next().ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected term after operator".to_string(),
        })?;
        let mut rhs = build_term(next_term_pair)?;

        // Apply trailing signs to the right-hand side (innermost first)
        for sign_pair in trailing_signs.into_iter().rev() {
            rhs = apply_sign(sign_pair, rhs)?;
        }

        // Build the binary expression, folding left
        result = build_binary_op(op_pair, result, rhs)?;
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Sign handling
// ---------------------------------------------------------------------------

/// Wrap `expr` in a `Sign` node based on the sign pair (`+` or `-`).
fn apply_sign(sign_pair: Pair<Rule>, expr: Expr) -> Result<Expr, ParseError> {
    let inner = sign_pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::InvalidExpression("Empty sign pair".to_string()))?;
    match inner.as_rule() {
        Rule::op_add => Ok(Expr::Sign(SignKind::Positive, Box::new(expr))),
        Rule::op_sub => Ok(Expr::Sign(SignKind::Negative, Box::new(expr))),
        other => Err(ParseError::InvalidExpression(format!(
            "Unexpected sign rule: {:?}",
            other
        ))),
    }
}

// ---------------------------------------------------------------------------
// Binary operator dispatch
// ---------------------------------------------------------------------------

/// Check whether a pest `Rule` variant represents a binary operator.
fn is_binary_op_rule(rule: Rule) -> bool {
    matches!(
        rule,
        Rule::op_add
            | Rule::op_sub
            | Rule::op_mul
            | Rule::op_div
            | Rule::op_eq
            | Rule::op_neq
            | Rule::op_lt
            | Rule::op_gt
            | Rule::op_lte
            | Rule::op_gte
            | Rule::op_approx
            | Rule::op_napprox
            | Rule::op_in
            | Rule::op_notin
            | Rule::op_cup
            | Rule::op_cap
            | Rule::op_subset
            | Rule::op_superset
            | Rule::op_forall
            | Rule::op_exists
            | Rule::op_implies
            | Rule::op_iff
            | Rule::op_parallel
            | Rule::op_perp
            | Rule::op_congruent
            | Rule::op_similar
            | Rule::op_to
    )
}

/// Build the correct AST binary-expression variant from an operator pair.
fn build_binary_op(op: Pair<Rule>, lhs: Expr, rhs: Expr) -> Result<Expr, ParseError> {
    let l = Box::new(lhs);
    let r = Box::new(rhs);
    Ok(match op.as_rule() {
        // Arithmetic operators
        Rule::op_add => Expr::BinaryOp(BinaryOpKind::Add, l, r),
        Rule::op_sub => Expr::BinaryOp(BinaryOpKind::Sub, l, r),
        Rule::op_mul => Expr::BinaryOp(BinaryOpKind::Mul, l, r),
        Rule::op_div => Expr::BinaryOp(BinaryOpKind::Div, l, r),
        // Comparison operators
        Rule::op_eq => Expr::ComparisonOp(ComparisonOpKind::Eq, l, r),
        Rule::op_neq => Expr::ComparisonOp(ComparisonOpKind::Neq, l, r),
        Rule::op_lt => Expr::ComparisonOp(ComparisonOpKind::Lt, l, r),
        Rule::op_gt => Expr::ComparisonOp(ComparisonOpKind::Gt, l, r),
        Rule::op_lte => Expr::ComparisonOp(ComparisonOpKind::Lte, l, r),
        Rule::op_gte => Expr::ComparisonOp(ComparisonOpKind::Gte, l, r),
        Rule::op_approx => Expr::ComparisonOp(ComparisonOpKind::Approx, l, r),
        Rule::op_napprox => Expr::ComparisonOp(ComparisonOpKind::Napprox, l, r),
        // Set operators
        Rule::op_in => Expr::SetOp(SetOpKind::In, l, r),
        Rule::op_notin => Expr::SetOp(SetOpKind::NotIn, l, r),
        Rule::op_cup => Expr::SetOp(SetOpKind::Cup, l, r),
        Rule::op_cap => Expr::SetOp(SetOpKind::Cap, l, r),
        Rule::op_subset => Expr::SetOp(SetOpKind::Subset, l, r),
        Rule::op_superset => Expr::SetOp(SetOpKind::Superset, l, r),
        // Logic operators
        Rule::op_forall => Expr::LogicOp(LogicOpKind::Forall, l, r),
        Rule::op_exists => Expr::LogicOp(LogicOpKind::Exists, l, r),
        Rule::op_implies => Expr::LogicOp(LogicOpKind::Implies, l, r),
        Rule::op_iff => Expr::LogicOp(LogicOpKind::Iff, l, r),
        // Geometry relation operators (mapped to ComparisonOp)
        Rule::op_parallel => Expr::ComparisonOp(ComparisonOpKind::Parallel, l, r),
        Rule::op_perp => Expr::ComparisonOp(ComparisonOpKind::Perpendicular, l, r),
        Rule::op_congruent => Expr::ComparisonOp(ComparisonOpKind::Congruent, l, r),
        Rule::op_similar => Expr::ComparisonOp(ComparisonOpKind::Similar, l, r),
        // Arrow operator (used in limit subscript bounds and standalone)
        Rule::op_to => Expr::ComparisonOp(ComparisonOpKind::To, l, r),
        _ => {
            return Err(ParseError::InvalidExpression(format!(
                "Unknown binary op rule: {:?}",
                op.as_rule()
            )))
        }
    })
}

// ---------------------------------------------------------------------------
// Term builder (primary + suffixes)
// ---------------------------------------------------------------------------

/// Build an `Expr` from a pest `term` pair.
///
/// Grammar: `term = { primary ~ suffix* }`
///
/// Suffixes wrap the base expression: `x^2` becomes
/// `Superscript(Identifier("x"), Number(2))`.
fn build_term(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let primary_pair = inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected primary in term".to_string(),
    })?;
    let mut result = build_primary(primary_pair)?;

    for suffix in inner {
        result = match suffix.as_rule() {
            Rule::superscript => {
                let sup = build_superscript_content(suffix)?;
                Expr::Superscript(Box::new(result), Box::new(sup))
            }
            Rule::subscript => {
                let sub = build_subscript_content(suffix)?;
                Expr::Subscript(Box::new(result), Box::new(sub))
            }
            Rule::degree_mark => Expr::Degree(Box::new(result)),
            Rule::percent_mark => Expr::Percent(Box::new(result)),
            _ => {
                return Err(ParseError::InvalidExpression(format!(
                    "Unexpected suffix rule in term: {:?}",
                    suffix.as_rule()
                )))
            }
        };
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Superscript / subscript content helpers
// ---------------------------------------------------------------------------

/// Extract the expression inside a `superscript` pair.
///
/// Grammar: `superscript = { "^" ~ (lbrace ~ expr ~ rbrace | superscript_atom) }`
///
/// Since `lbrace`/`rbrace` and `superscript_atom` are silent rules,
/// the inner pair is either `expr` or an atomic rule (`number`/`identifier`/`greek`).
fn build_superscript_content(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected content in superscript".to_string(),
        })?;
    match inner.as_rule() {
        Rule::expr => build_expr(inner),
        Rule::number => parse_number(inner),
        Rule::identifier => Ok(Expr::Identifier(inner.as_str().to_string())),
        Rule::greek => Ok(build_greek(inner)),
        _ => Err(ParseError::InvalidExpression(format!(
            "Unexpected superscript content: {:?}",
            inner.as_rule()
        ))),
    }
}

/// Extract the expression inside a `subscript` pair.
///
/// Grammar: `subscript = { "_" ~ (lbrace ~ expr ~ rbrace | subscript_atom) }`
fn build_subscript_content(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected content in subscript".to_string(),
        })?;
    match inner.as_rule() {
        Rule::expr => build_expr(inner),
        Rule::number => parse_number(inner),
        Rule::identifier => Ok(Expr::Identifier(inner.as_str().to_string())),
        Rule::greek => Ok(build_greek(inner)),
        _ => Err(ParseError::InvalidExpression(format!(
            "Unexpected subscript content: {:?}",
            inner.as_rule()
        ))),
    }
}

// ---------------------------------------------------------------------------
// Primary builder (dispatches to specific builders)
// ---------------------------------------------------------------------------

/// Build an `Expr` from a pest `primary` pair.
///
/// The `primary` rule is an ordered choice; the pair wraps exactly one
/// child matching the winning alternative.
fn build_primary(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let content = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected content in primary".to_string(),
        })?;
    build_primary_content(content)
}

/// Dispatch on the inner rule of a `primary` pair.
fn build_primary_content(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    match pair.as_rule() {
        Rule::number => parse_number(pair),
        Rule::identifier => Ok(Expr::Identifier(pair.as_str().to_string())),
        Rule::greek => Ok(build_greek(pair)),
        Rule::group => build_group(pair),
        Rule::frac => build_frac(pair),
        Rule::sqrt => build_sqrt(pair),
        Rule::sqrt_n => build_sqrt_n(pair),
        Rule::function => build_function(pair),
        Rule::pm => build_pm(pair),
        Rule::emptyset => Ok(Expr::Emptyset),
        Rule::geometry => build_geometry(pair),
        Rule::limit_expr => build_limit(pair),
        Rule::sum_expr => build_sum(pair),
        Rule::product_expr => build_product(pair),
        Rule::integral_expr => build_integral(pair),
        _ => Err(ParseError::InvalidExpression(format!(
            "Unexpected primary content rule: {:?}",
            pair.as_rule()
        ))),
    }
}

// ---------------------------------------------------------------------------
// Atom builders
// ---------------------------------------------------------------------------

/// Parse a `number` pair into `Expr::Number`.
fn parse_number(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    pair.as_str().parse::<f64>().map(Expr::Number).map_err(|e| {
        ParseError::InvalidExpression(format!("Invalid number '{}': {}", pair.as_str(), e))
    })
}

/// Build a `Greek` expression from a `greek` pair.
fn build_greek(pair: Pair<Rule>) -> Expr {
    Expr::Greek(match pair.as_str() {
        "\\alpha" => GreekLetter::Alpha,
        "\\beta" => GreekLetter::Beta,
        "\\gamma" => GreekLetter::Gamma,
        "\\delta" => GreekLetter::Delta,
        "\\epsilon" => GreekLetter::Epsilon,
        "\\theta" => GreekLetter::Theta,
        "\\lambda" => GreekLetter::Lambda,
        "\\mu" => GreekLetter::Mu,
        "\\sigma" => GreekLetter::Sigma,
        "\\pi" => GreekLetter::Pi,
        "\\phi" => GreekLetter::Phi,
        "\\omega" => GreekLetter::Omega,
        other => unreachable!("Unknown greek letter: {}", other),
    })
}

// ---------------------------------------------------------------------------
// Group builder
// ---------------------------------------------------------------------------

/// Build a `Group` expression from a `group` pair.
///
/// Grammar: `group = { lparen ~ expr ~ rparen }`
fn build_group(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let expr_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected expression in group".to_string(),
        })?;
    Ok(Expr::Group(Box::new(build_expr(expr_pair)?)))
}

// ---------------------------------------------------------------------------
// Fraction builder
// ---------------------------------------------------------------------------

/// Build a `Frac` expression.
///
/// Grammar: `frac = { cmd_frac ~ lbrace ~ expr ~ rbrace ~ lbrace ~ expr ~ rbrace }`
fn build_frac(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let num = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected numerator in fraction".to_string(),
    })?)?;
    let den = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected denominator in fraction".to_string(),
    })?)?;
    Ok(Expr::Frac(Box::new(num), Box::new(den)))
}

// ---------------------------------------------------------------------------
// Root builders
// ---------------------------------------------------------------------------

/// Build a `Sqrt` expression.
///
/// Grammar: `sqrt = { cmd_sqrt ~ lbrace ~ expr ~ rbrace }`
fn build_sqrt(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let expr_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected expression in square root".to_string(),
        })?;
    Ok(Expr::Sqrt(Box::new(build_expr(expr_pair)?)))
}

/// Build a `SqrtN` expression (nth root).
///
/// Grammar: `sqrt_n = { cmd_sqrt ~ lbracket ~ expr ~ rbracket ~ lbrace ~ expr ~ rbrace }`
fn build_sqrt_n(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let degree = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected degree in nth root".to_string(),
    })?)?;
    let radicand = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected radicand in nth root".to_string(),
    })?)?;
    Ok(Expr::SqrtN(Box::new(degree), Box::new(radicand)))
}

// ---------------------------------------------------------------------------
// Plus/minus builder
// ---------------------------------------------------------------------------

/// Build a `Pm` or `Mp` expression.
///
/// Grammar: `pm = { (cmd_pm | cmd_mp) ~ term }`
///
/// `cmd_pm` / `cmd_mp` are silent rules, so we inspect the raw text
/// to determine which variant was matched.
fn build_pm(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let raw = pair.as_str();
    let term_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected term after plus-minus".to_string(),
        })?;
    let term_expr = build_term(term_pair)?;
    if raw.starts_with("\\pm") || raw.starts_with("\u{00b1}") {
        Ok(Expr::Pm(Box::new(term_expr)))
    } else if raw.starts_with("\\mp") || raw.starts_with("\u{2213}") {
        Ok(Expr::Mp(Box::new(term_expr)))
    } else {
        Err(ParseError::InvalidExpression(format!(
            "Unknown plus/minus variant in: {}",
            raw
        )))
    }
}

// ---------------------------------------------------------------------------
// Function builder
// ---------------------------------------------------------------------------

/// Build a `Function` expression.
///
/// Grammar: `function = { cmd_func ~ (lbrace ~ expr ~ rbrace | group | primary) }`
///
/// `cmd_func` is a silent rule; the function name is determined from the
/// raw text of the pair.
fn build_function(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let raw = pair.as_str();
    let arg_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::SyntaxError {
            position: 0,
            message: "Expected argument in function".to_string(),
        })?;

    // Build the argument expression depending on which alternative matched
    let arg_expr = match arg_pair.as_rule() {
        Rule::expr => build_expr(arg_pair)?,
        Rule::group => {
            let inner = arg_pair
                .into_inner()
                .next()
                .ok_or_else(|| ParseError::SyntaxError {
                    position: 0,
                    message: "Expected expression in group argument".to_string(),
                })?;
            build_expr(inner)?
        }
        Rule::primary => build_primary(arg_pair)?,
        _ => {
            return Err(ParseError::InvalidExpression(format!(
                "Unexpected function argument rule: {:?}",
                arg_pair.as_rule()
            )))
        }
    };

    // Determine which function from the raw text prefix
    let func_kind = if raw.starts_with("\\sin") {
        MathFunction::Sin
    } else if raw.starts_with("\\cos") {
        MathFunction::Cos
    } else if raw.starts_with("\\tan") {
        MathFunction::Tan
    } else if raw.starts_with("\\cot") {
        MathFunction::Cot
    } else if raw.starts_with("\\sec") {
        MathFunction::Sec
    } else if raw.starts_with("\\csc") {
        MathFunction::Csc
    } else if raw.starts_with("\\log") {
        MathFunction::Log
    } else if raw.starts_with("\\ln") {
        MathFunction::Ln
    } else if raw.starts_with("\\lg") {
        MathFunction::Lg
    } else {
        return Err(ParseError::InvalidExpression(format!(
            "Unknown function prefix in: {}",
            raw
        )));
    };

    Ok(Expr::Function(func_kind, Box::new(arg_expr)))
}

// ---------------------------------------------------------------------------
// Geometry symbol builder
// ---------------------------------------------------------------------------

/// Build a `Geometry` expression from a standalone geometry symbol.
///
/// Grammar: `geometry = { cmd_triangle | cmd_angle }`
fn build_geometry(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    Ok(Expr::Geometry(match pair.as_str() {
        "\\triangle" => GeometrySymbol::Triangle,
        "\\angle" => GeometrySymbol::Angle,
        _ => {
            return Err(ParseError::InvalidExpression(format!(
                "Unknown geometry symbol: {}",
                pair.as_str()
            )))
        }
    }))
}

// ---------------------------------------------------------------------------
// Calculus builders
// ---------------------------------------------------------------------------

/// Build a `Limit` expression.
///
/// Grammar: `limit_expr = { cmd_lim ~ subscript_bounds ~ term }`
///
/// The subscript_bounds contains an expression like `x \to 0`, which
/// is parsed as `ComparisonOp(To, var, target)`. We extract the
/// variable and target to construct `Limit(var, target, body)`.
fn build_limit(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let bounds_expr = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected subscript bounds in limit".to_string(),
    })?)?;
    let body = build_term(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected body term in limit".to_string(),
    })?)?;
    let (var, target) = extract_limit_bounds(bounds_expr)?;
    Ok(Expr::Limit(Box::new(var), Box::new(target), Box::new(body)))
}

/// Extract the variable and target from a limit subscript expression.
///
/// Expected form: `ComparisonOp(To, variable, target)`
fn extract_limit_bounds(expr: Expr) -> Result<(Expr, Expr), ParseError> {
    match expr {
        Expr::ComparisonOp(ComparisonOpKind::To, var, target) => Ok((*var, *target)),
        other => Err(ParseError::InvalidExpression(format!(
            "Expected 'var -> target' in limit subscript, got {:?}",
            other
        ))),
    }
}

/// Build a `Sum` expression.
///
/// Grammar: `sum_expr = { cmd_sum ~ subscript_bounds ~ superscript_bounds ~ term }`
///
/// The subscript_bounds contains `var = lower`, and superscript_bounds
/// contains the upper bound.
fn build_sum(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let sub_expr = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected subscript bounds in sum".to_string(),
    })?)?;
    let sup_expr = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected superscript bounds in sum".to_string(),
    })?)?;
    let body = build_term(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected body term in sum".to_string(),
    })?)?;
    let (var, lower) = extract_sum_bounds(sub_expr)?;
    Ok(Expr::Sum(
        Box::new(var),
        Box::new(lower),
        Box::new(sup_expr),
        Box::new(body),
    ))
}

/// Build a `Product` expression.
///
/// Grammar: `product_expr = { cmd_prod ~ subscript_bounds ~ superscript_bounds ~ term }`
fn build_product(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let sub_expr = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected subscript bounds in product".to_string(),
    })?)?;
    let sup_expr = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected superscript bounds in product".to_string(),
    })?)?;
    let body = build_term(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected body term in product".to_string(),
    })?)?;
    let (var, lower) = extract_sum_bounds(sub_expr)?;
    Ok(Expr::Product(
        Box::new(var),
        Box::new(lower),
        Box::new(sup_expr),
        Box::new(body),
    ))
}

/// Build an `Integral` expression.
///
/// Grammar: `integral_expr = { cmd_int ~ subscript_bounds ~ superscript_bounds ~ term }`
fn build_integral(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let lower = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected subscript bounds in integral".to_string(),
    })?)?;
    let upper = build_expr(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected superscript bounds in integral".to_string(),
    })?)?;
    let body = build_term(inner.next().ok_or_else(|| ParseError::SyntaxError {
        position: 0,
        message: "Expected body term in integral".to_string(),
    })?)?;
    Ok(Expr::Integral(
        Box::new(lower),
        Box::new(upper),
        Box::new(body),
    ))
}

/// Extract the variable and lower bound from a sum/product subscript expression.
///
/// Expected form: `ComparisonOp(Eq, variable, lower_bound)`
fn extract_sum_bounds(expr: Expr) -> Result<(Expr, Expr), ParseError> {
    match expr {
        Expr::ComparisonOp(ComparisonOpKind::Eq, var, lower) => Ok((*var, *lower)),
        other => Err(ParseError::InvalidExpression(format!(
            "Expected 'var = lower' in sum/product subscript, got {:?}",
            other
        ))),
    }
}
