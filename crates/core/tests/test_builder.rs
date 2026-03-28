use latex2chin_core::ast::*;
use latex2chin_core::builder::parse_to_ast;

// ── Numbers ──────────────────────────────────────────────────────────────────

#[test]
fn ast_number_integer() {
    assert_eq!(parse_to_ast("42").unwrap(), Expr::Number(42.0));
}

#[test]
fn ast_number_decimal() {
    assert_eq!(parse_to_ast("3.14").unwrap(), Expr::Number(3.14));
}

// ── Identifiers ──────────────────────────────────────────────────────────────

#[test]
fn ast_identifier() {
    assert_eq!(
        parse_to_ast("x").unwrap(),
        Expr::Identifier("x".to_string())
    );
}

// ── Greek letters ────────────────────────────────────────────────────────────

#[test]
fn ast_greek_alpha() {
    assert_eq!(
        parse_to_ast("\\alpha").unwrap(),
        Expr::Greek(GreekLetter::Alpha)
    );
}

#[test]
fn ast_greek_pi() {
    assert_eq!(parse_to_ast("\\pi").unwrap(), Expr::Greek(GreekLetter::Pi));
}

// ── Signs ────────────────────────────────────────────────────────────────────

#[test]
fn ast_positive_sign() {
    assert_eq!(
        parse_to_ast("+5").unwrap(),
        Expr::Sign(SignKind::Positive, Box::new(Expr::Number(5.0)))
    );
}

#[test]
fn ast_negative_sign() {
    assert_eq!(
        parse_to_ast("-3").unwrap(),
        Expr::Sign(SignKind::Negative, Box::new(Expr::Number(3.0)))
    );
}

// ── Arithmetic ───────────────────────────────────────────────────────────────

#[test]
fn ast_addition() {
    assert_eq!(
        parse_to_ast("1 + 2").unwrap(),
        Expr::BinaryOp(
            BinaryOpKind::Add,
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Number(2.0))
        )
    );
}

#[test]
fn ast_subtraction() {
    assert_eq!(
        parse_to_ast("5 - 3").unwrap(),
        Expr::BinaryOp(
            BinaryOpKind::Sub,
            Box::new(Expr::Number(5.0)),
            Box::new(Expr::Number(3.0))
        )
    );
}

#[test]
fn ast_multiplication() {
    assert_eq!(
        parse_to_ast("2 * 3").unwrap(),
        Expr::BinaryOp(
            BinaryOpKind::Mul,
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Number(3.0))
        )
    );
}

#[test]
fn ast_division() {
    assert_eq!(
        parse_to_ast("6 / 2").unwrap(),
        Expr::BinaryOp(
            BinaryOpKind::Div,
            Box::new(Expr::Number(6.0)),
            Box::new(Expr::Number(2.0))
        )
    );
}

// ── Left-to-right associativity ──────────────────────────────────────────────

#[test]
fn ast_left_to_right_add_sub() {
    // 1 + 2 - 3 => (1 + 2) - 3
    assert_eq!(
        parse_to_ast("1 + 2 - 3").unwrap(),
        Expr::BinaryOp(
            BinaryOpKind::Sub,
            Box::new(Expr::BinaryOp(
                BinaryOpKind::Add,
                Box::new(Expr::Number(1.0)),
                Box::new(Expr::Number(2.0))
            )),
            Box::new(Expr::Number(3.0))
        )
    );
}

#[test]
fn ast_left_to_right_chain() {
    // 1 + 2 * 3 - 4 / 5 => ((1 + 2) * 3 - 4) / 5 (flat left-to-right)
    let inner = Expr::BinaryOp(
        BinaryOpKind::Sub,
        Box::new(Expr::BinaryOp(
            BinaryOpKind::Mul,
            Box::new(Expr::BinaryOp(
                BinaryOpKind::Add,
                Box::new(Expr::Number(1.0)),
                Box::new(Expr::Number(2.0)),
            )),
            Box::new(Expr::Number(3.0)),
        )),
        Box::new(Expr::Number(4.0)),
    );
    let expected = Expr::BinaryOp(
        BinaryOpKind::Div,
        Box::new(inner),
        Box::new(Expr::Number(5.0)),
    );
    assert_eq!(parse_to_ast("1 + 2 * 3 - 4 / 5").unwrap(), expected);
}

// ── Comparison operators ─────────────────────────────────────────────────────

#[test]
fn ast_equality() {
    assert_eq!(
        parse_to_ast("1 = 2").unwrap(),
        Expr::ComparisonOp(
            ComparisonOpKind::Eq,
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Number(2.0))
        )
    );
}

#[test]
fn ast_less_than() {
    assert_eq!(
        parse_to_ast("1 < 2").unwrap(),
        Expr::ComparisonOp(
            ComparisonOpKind::Lt,
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Number(2.0))
        )
    );
}

#[test]
fn ast_approx() {
    assert_eq!(
        parse_to_ast("\\pi \\approx 3.14").unwrap(),
        Expr::ComparisonOp(
            ComparisonOpKind::Approx,
            Box::new(Expr::Greek(GreekLetter::Pi)),
            Box::new(Expr::Number(3.14))
        )
    );
}

// ── Set operators ────────────────────────────────────────────────────────────

#[test]
fn ast_set_in() {
    assert_eq!(
        parse_to_ast("x \\in A").unwrap(),
        Expr::SetOp(
            SetOpKind::In,
            Box::new(Expr::Identifier("x".to_string())),
            Box::new(Expr::Identifier("A".to_string()))
        )
    );
}

#[test]
fn ast_set_cup() {
    assert_eq!(
        parse_to_ast("A \\cup B").unwrap(),
        Expr::SetOp(
            SetOpKind::Cup,
            Box::new(Expr::Identifier("A".to_string())),
            Box::new(Expr::Identifier("B".to_string()))
        )
    );
}

// ── Logic operators ──────────────────────────────────────────────────────────

#[test]
fn ast_logic_implies() {
    assert_eq!(
        parse_to_ast("A \\Rightarrow B").unwrap(),
        Expr::LogicOp(
            LogicOpKind::Implies,
            Box::new(Expr::Identifier("A".to_string())),
            Box::new(Expr::Identifier("B".to_string()))
        )
    );
}

#[test]
fn ast_logic_iff() {
    assert_eq!(
        parse_to_ast("A \\iff B").unwrap(),
        Expr::LogicOp(
            LogicOpKind::Iff,
            Box::new(Expr::Identifier("A".to_string())),
            Box::new(Expr::Identifier("B".to_string()))
        )
    );
}

// ── Geometry relation operators ──────────────────────────────────────────────

#[test]
fn ast_parallel() {
    assert_eq!(
        parse_to_ast("A \\parallel B").unwrap(),
        Expr::ComparisonOp(
            ComparisonOpKind::Parallel,
            Box::new(Expr::Identifier("A".to_string())),
            Box::new(Expr::Identifier("B".to_string()))
        )
    );
}

#[test]
fn ast_perp() {
    assert_eq!(
        parse_to_ast("A \\perp B").unwrap(),
        Expr::ComparisonOp(
            ComparisonOpKind::Perpendicular,
            Box::new(Expr::Identifier("A".to_string())),
            Box::new(Expr::Identifier("B".to_string()))
        )
    );
}

// ── Arrow (to) operator ──────────────────────────────────────────────────────

#[test]
fn ast_to_operator() {
    assert_eq!(
        parse_to_ast("x \\to 0").unwrap(),
        Expr::ComparisonOp(
            ComparisonOpKind::To,
            Box::new(Expr::Identifier("x".to_string())),
            Box::new(Expr::Number(0.0))
        )
    );
}

// ── Groups (parenthesized) ───────────────────────────────────────────────────

#[test]
fn ast_group() {
    assert_eq!(
        parse_to_ast("(1 + 2)").unwrap(),
        Expr::Group(Box::new(Expr::BinaryOp(
            BinaryOpKind::Add,
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Number(2.0))
        )))
    );
}

// ── Fractions ────────────────────────────────────────────────────────────────

#[test]
fn ast_frac() {
    assert_eq!(
        parse_to_ast("\\frac{1}{2}").unwrap(),
        Expr::Frac(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)))
    );
}

#[test]
fn ast_frac_with_negative() {
    assert_eq!(
        parse_to_ast("\\frac{-1}{2}").unwrap(),
        Expr::Frac(
            Box::new(Expr::Sign(SignKind::Negative, Box::new(Expr::Number(1.0)))),
            Box::new(Expr::Number(2.0))
        )
    );
}

// ── Square root ──────────────────────────────────────────────────────────────

#[test]
fn ast_sqrt() {
    assert_eq!(
        parse_to_ast("\\sqrt{2}").unwrap(),
        Expr::Sqrt(Box::new(Expr::Number(2.0)))
    );
}

// ── Nth root ─────────────────────────────────────────────────────────────────

#[test]
fn ast_sqrt_n() {
    assert_eq!(
        parse_to_ast("\\sqrt[3]{8}").unwrap(),
        Expr::SqrtN(Box::new(Expr::Number(3.0)), Box::new(Expr::Number(8.0)))
    );
}

// ── Superscript ──────────────────────────────────────────────────────────────

#[test]
fn ast_superscript_simple() {
    assert_eq!(
        parse_to_ast("x^2").unwrap(),
        Expr::Superscript(
            Box::new(Expr::Identifier("x".to_string())),
            Box::new(Expr::Number(2.0))
        )
    );
}

#[test]
fn ast_superscript_braced() {
    assert_eq!(
        parse_to_ast("x^{n+1}").unwrap(),
        Expr::Superscript(
            Box::new(Expr::Identifier("x".to_string())),
            Box::new(Expr::BinaryOp(
                BinaryOpKind::Add,
                Box::new(Expr::Identifier("n".to_string())),
                Box::new(Expr::Number(1.0))
            ))
        )
    );
}

// ── Subscript ────────────────────────────────────────────────────────────────

#[test]
fn ast_subscript_simple() {
    assert_eq!(
        parse_to_ast("a_1").unwrap(),
        Expr::Subscript(
            Box::new(Expr::Identifier("a".to_string())),
            Box::new(Expr::Number(1.0))
        )
    );
}

#[test]
fn ast_subscript_braced() {
    assert_eq!(
        parse_to_ast("a_{n+1}").unwrap(),
        Expr::Subscript(
            Box::new(Expr::Identifier("a".to_string())),
            Box::new(Expr::BinaryOp(
                BinaryOpKind::Add,
                Box::new(Expr::Identifier("n".to_string())),
                Box::new(Expr::Number(1.0))
            ))
        )
    );
}

// ── Degree and percent ───────────────────────────────────────────────────────

#[test]
fn ast_degree() {
    assert_eq!(
        parse_to_ast("100\\degree").unwrap(),
        Expr::Degree(Box::new(Expr::Number(100.0)))
    );
}

#[test]
fn ast_percent() {
    assert_eq!(
        parse_to_ast("50%").unwrap(),
        Expr::Percent(Box::new(Expr::Number(50.0)))
    );
}

// ── Plus/minus ───────────────────────────────────────────────────────────────

#[test]
fn ast_pm() {
    assert_eq!(
        parse_to_ast("\\pm 2").unwrap(),
        Expr::Pm(Box::new(Expr::Number(2.0)))
    );
}

#[test]
fn ast_mp() {
    assert_eq!(
        parse_to_ast("\\mp 2").unwrap(),
        Expr::Mp(Box::new(Expr::Number(2.0)))
    );
}

// ── Functions ────────────────────────────────────────────────────────────────

#[test]
fn ast_sin() {
    assert_eq!(
        parse_to_ast("\\sin x").unwrap(),
        Expr::Function(
            MathFunction::Sin,
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

#[test]
fn ast_cos() {
    assert_eq!(
        parse_to_ast("\\cos x").unwrap(),
        Expr::Function(
            MathFunction::Cos,
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

#[test]
fn ast_sin_braced() {
    assert_eq!(
        parse_to_ast("\\sin{x}").unwrap(),
        Expr::Function(
            MathFunction::Sin,
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

#[test]
fn ast_sin_group() {
    assert_eq!(
        parse_to_ast("\\sin(x + 1)").unwrap(),
        Expr::Function(
            MathFunction::Sin,
            Box::new(Expr::BinaryOp(
                BinaryOpKind::Add,
                Box::new(Expr::Identifier("x".to_string())),
                Box::new(Expr::Number(1.0))
            ))
        )
    );
}

#[test]
fn ast_log() {
    assert_eq!(
        parse_to_ast("\\log x").unwrap(),
        Expr::Function(
            MathFunction::Log,
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

#[test]
fn ast_ln() {
    assert_eq!(
        parse_to_ast("\\ln x").unwrap(),
        Expr::Function(
            MathFunction::Ln,
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

#[test]
fn ast_lg() {
    assert_eq!(
        parse_to_ast("\\lg x").unwrap(),
        Expr::Function(
            MathFunction::Lg,
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

// ── Geometry standalone symbols ──────────────────────────────────────────────

#[test]
fn ast_triangle() {
    assert_eq!(
        parse_to_ast("\\triangle").unwrap(),
        Expr::Geometry(GeometrySymbol::Triangle)
    );
}

#[test]
fn ast_angle() {
    assert_eq!(
        parse_to_ast("\\angle").unwrap(),
        Expr::Geometry(GeometrySymbol::Angle)
    );
}

// ── Emptyset ─────────────────────────────────────────────────────────────────

#[test]
fn ast_emptyset() {
    assert_eq!(parse_to_ast("\\emptyset").unwrap(), Expr::Emptyset);
}

// ── Limit ────────────────────────────────────────────────────────────────────

#[test]
fn ast_limit() {
    assert_eq!(
        parse_to_ast("\\lim_{x \\to 0} x").unwrap(),
        Expr::Limit(
            Box::new(Expr::Identifier("x".to_string())),
            Box::new(Expr::Number(0.0)),
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

// ── Sum ──────────────────────────────────────────────────────────────────────

#[test]
fn ast_sum() {
    assert_eq!(
        parse_to_ast("\\sum_{i=1}^{n} i").unwrap(),
        Expr::Sum(
            Box::new(Expr::Identifier("i".to_string())),
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Identifier("n".to_string())),
            Box::new(Expr::Identifier("i".to_string()))
        )
    );
}

// ── Product ──────────────────────────────────────────────────────────────────

#[test]
fn ast_product() {
    assert_eq!(
        parse_to_ast("\\prod_{i=1}^{n} i").unwrap(),
        Expr::Product(
            Box::new(Expr::Identifier("i".to_string())),
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Identifier("n".to_string())),
            Box::new(Expr::Identifier("i".to_string()))
        )
    );
}

// ── Integral ─────────────────────────────────────────────────────────────────

#[test]
fn ast_integral() {
    assert_eq!(
        parse_to_ast("\\int_{0}^{1} x").unwrap(),
        Expr::Integral(
            Box::new(Expr::Number(0.0)),
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Identifier("x".to_string()))
        )
    );
}

// ── Nested complex expression ────────────────────────────────────────────────

#[test]
fn ast_nested_frac_sqrt_superscript() {
    let result = parse_to_ast("\\frac{\\sqrt{x^2+1}}{2}").unwrap();

    let x_sq = Expr::Superscript(
        Box::new(Expr::Identifier("x".to_string())),
        Box::new(Expr::Number(2.0)),
    );
    let x_sq_plus_1 = Expr::BinaryOp(
        BinaryOpKind::Add,
        Box::new(x_sq),
        Box::new(Expr::Number(1.0)),
    );
    let sqrt_expr = Expr::Sqrt(Box::new(x_sq_plus_1));
    let expected = Expr::Frac(Box::new(sqrt_expr), Box::new(Expr::Number(2.0)));

    assert_eq!(result, expected);
}

// ── Sign wrapping in expression context ──────────────────────────────────────

#[test]
fn ast_negative_in_binary_op() {
    // 1 + -2 => BinaryOp(Add, 1, Sign(Negative, 2))
    assert_eq!(
        parse_to_ast("1 + -2").unwrap(),
        Expr::BinaryOp(
            BinaryOpKind::Add,
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Sign(SignKind::Negative, Box::new(Expr::Number(2.0))))
        )
    );
}

// ── Error on invalid input ───────────────────────────────────────────────────

#[test]
fn ast_reject_empty() {
    assert!(parse_to_ast("").is_err());
}

#[test]
fn ast_reject_bare_operator() {
    assert!(parse_to_ast("+").is_err());
}
