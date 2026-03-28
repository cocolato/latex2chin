/// AST to Chinese text translator.
///
/// Traverses an `Expr` AST produced by `builder::parse_to_ast()` and
/// produces a Chinese-language reading of the mathematical expression.
/// Special-case conventions are applied for squares, cubes, and other
/// common patterns.
use crate::ast::*;

/// Translate an AST expression into its Chinese reading.
pub fn translate(expr: &Expr) -> String {
    match expr {
        // -- Atoms ----------------------------------------------------------
        Expr::Number(n) => format_number(*n),
        Expr::Identifier(s) => s.clone(),
        Expr::Greek(g) => translate_greek(g).to_string(),

        // -- Arithmetic binary operators ------------------------------------
        Expr::BinaryOp(kind, lhs, rhs) => {
            format!(
                "{}{}{}",
                translate(lhs),
                translate_binary_op(kind),
                translate(rhs)
            )
        }

        // -- Comparison binary operators ------------------------------------
        Expr::ComparisonOp(kind, lhs, rhs) => {
            format!(
                "{}{}{}",
                translate(lhs),
                translate_comparison_op(kind),
                translate(rhs)
            )
        }

        // -- Fractions (Chinese: denominator first) -------------------------
        Expr::Frac(num, den) => {
            format!("{}分之{}", translate(den), translate(num))
        }

        // -- Roots ----------------------------------------------------------
        Expr::Sqrt(radicand) => format!("根号{}", translate(radicand)),
        Expr::SqrtN(degree, radicand) => {
            format!("{}次根号{}", translate(degree), translate(radicand))
        }

        // -- Superscript (special cases for square and cube) ----------------
        Expr::Superscript(base, exp) => match exp.as_ref() {
            Expr::Number(n) if *n == 2.0 => format!("{}的平方", translate(base)),
            Expr::Number(n) if *n == 3.0 => format!("{}的立方", translate(base)),
            _ => format!("{}的{}次方", translate(base), translate(exp)),
        },

        // -- Subscript ------------------------------------------------------
        Expr::Subscript(base, sub) => {
            format!("{}下标{}", translate(base), translate(sub))
        }

        // -- Functions (Chinese names) --------------------------------------
        Expr::Function(func, arg) => {
            format!("{}{}", translate_function(func), translate(arg))
        }

        // -- Group (parentheses dropped) ------------------------------------
        Expr::Group(inner) => translate(inner),

        // -- Plus/minus, minus/plus -----------------------------------------
        Expr::Pm(inner) => format!("正负{}", translate(inner)),
        Expr::Mp(inner) => format!("负正{}", translate(inner)),

        // -- Degree / Percent suffixes --------------------------------------
        Expr::Degree(inner) => format!("{}度", translate(inner)),
        Expr::Percent(inner) => format!("百分之{}", translate(inner)),

        // -- Sign prefix ----------------------------------------------------
        Expr::Sign(kind, inner) => match kind {
            SignKind::Positive => format!("正{}", translate(inner)),
            SignKind::Negative => format!("负{}", translate(inner)),
        },

        // -- Calculus constructs --------------------------------------------
        Expr::Limit(var, target, body) => {
            format!(
                "当{}趋向于{}时的极限{}",
                translate(var),
                translate(target),
                translate(body)
            )
        }
        Expr::Sum(var, lower, upper, body) => {
            format!(
                "{}从{}到{}求和{}",
                translate(var),
                translate(lower),
                translate(upper),
                translate(body)
            )
        }
        Expr::Product(var, lower, upper, body) => {
            format!(
                "{}从{}到{}求积{}",
                translate(var),
                translate(lower),
                translate(upper),
                translate(body)
            )
        }
        Expr::Integral(lower, upper, body) => {
            format!(
                "从{}到{}积分{}",
                translate(lower),
                translate(upper),
                translate(body)
            )
        }

        // -- Set operators --------------------------------------------------
        Expr::SetOp(kind, lhs, rhs) => {
            format!(
                "{}{}{}",
                translate(lhs),
                translate_set_op(kind),
                translate(rhs)
            )
        }

        // -- Logic operators ------------------------------------------------
        Expr::LogicOp(kind, lhs, rhs) => {
            format!(
                "{}{}{}",
                translate(lhs),
                translate_logic_op(kind),
                translate(rhs)
            )
        }

        // -- Geometry symbols -----------------------------------------------
        Expr::Geometry(sym) => translate_geometry(sym).to_string(),
    }
}

// ---------------------------------------------------------------------------
// Formatting helpers
// ---------------------------------------------------------------------------

/// Format an f64 number: integers without decimal point, decimals as-is.
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

/// Translate a Greek letter to its standard Chinese transliteration.
fn translate_greek(g: &GreekLetter) -> &'static str {
    match g {
        GreekLetter::Alpha => "阿尔法",
        GreekLetter::Beta => "贝塔",
        GreekLetter::Gamma => "伽马",
        GreekLetter::Delta => "德尔塔",
        GreekLetter::Epsilon => "艾普西龙",
        GreekLetter::Theta => "西塔",
        GreekLetter::Lambda => "兰姆达",
        GreekLetter::Mu => "缪",
        GreekLetter::Sigma => "西格玛",
        GreekLetter::Pi => "派",
        GreekLetter::Phi => "fai",
        GreekLetter::Omega => "欧米伽",
    }
}

/// Translate an arithmetic binary operator to Chinese.
fn translate_binary_op(kind: &BinaryOpKind) -> &'static str {
    match kind {
        BinaryOpKind::Add => "加",
        BinaryOpKind::Sub => "减",
        BinaryOpKind::Mul => "乘",
        BinaryOpKind::Div => "除以",
    }
}

/// Translate a comparison operator to Chinese.
fn translate_comparison_op(kind: &ComparisonOpKind) -> &'static str {
    match kind {
        ComparisonOpKind::Eq => "等于",
        ComparisonOpKind::Neq => "不等于",
        ComparisonOpKind::Lt => "小于",
        ComparisonOpKind::Gt => "大于",
        ComparisonOpKind::Lte => "小于等于",
        ComparisonOpKind::Gte => "大于等于",
        ComparisonOpKind::Approx => "约等于",
        ComparisonOpKind::Napprox => "不约等于",
        ComparisonOpKind::To => "趋向于",
        ComparisonOpKind::Parallel => "平行于",
        ComparisonOpKind::Perpendicular => "垂直于",
        ComparisonOpKind::Congruent => "全等于",
        ComparisonOpKind::Similar => "相似于",
    }
}

/// Translate a mathematical function name to Chinese.
fn translate_function(func: &MathFunction) -> &'static str {
    match func {
        MathFunction::Sin => "正弦",
        MathFunction::Cos => "余弦",
        MathFunction::Tan => "正切",
        MathFunction::Cot => "余切",
        MathFunction::Sec => "正割",
        MathFunction::Csc => "余割",
        MathFunction::Log => "对数",
        MathFunction::Ln => "自然对数",
        MathFunction::Lg => "常用对数",
    }
}

/// Translate a set theory operator to Chinese.
fn translate_set_op(kind: &SetOpKind) -> &'static str {
    match kind {
        SetOpKind::In => "属于",
        SetOpKind::NotIn => "不属于",
        SetOpKind::Cup => "并",
        SetOpKind::Cap => "交",
        SetOpKind::Subset => "真子集",
        SetOpKind::Superset => "真超集",
        SetOpKind::Emptyset => "空集",
    }
}

/// Translate a logic operator to Chinese.
fn translate_logic_op(kind: &LogicOpKind) -> &'static str {
    match kind {
        LogicOpKind::Forall => "任意",
        LogicOpKind::Exists => "存在",
        LogicOpKind::Implies => "推出",
        LogicOpKind::Iff => "等价于",
    }
}

/// Translate a geometry symbol to Chinese.
fn translate_geometry(sym: &GeometrySymbol) -> &'static str {
    match sym {
        GeometrySymbol::Triangle => "三角形",
        GeometrySymbol::Angle => "角",
        GeometrySymbol::Parallel => "平行",
        GeometrySymbol::Perpendicular => "垂直",
        GeometrySymbol::Congruent => "全等",
        GeometrySymbol::Similar => "相似",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to box an expression
    fn box_expr(e: Expr) -> Box<Expr> {
        Box::new(e)
    }

    // -- Numbers -----------------------------------------------------------
    #[test]
    fn translate_number_integer() {
        assert_eq!(translate(&Expr::Number(42.0)), "42");
    }

    #[test]
    fn translate_number_decimal() {
        assert_eq!(translate(&Expr::Number(3.14)), "3.14");
    }

    #[test]
    fn translate_number_zero() {
        assert_eq!(translate(&Expr::Number(0.0)), "0");
    }

    #[test]
    fn translate_number_one() {
        assert_eq!(translate(&Expr::Number(1.0)), "1");
    }

    #[test]
    fn translate_number_negative_int() {
        // Negative numbers are wrapped in Sign, so Number holds the magnitude
        assert_eq!(translate(&Expr::Number(-5.0)), "-5");
    }

    // -- Identifiers -------------------------------------------------------
    #[test]
    fn translate_identifier() {
        assert_eq!(translate(&Expr::Identifier("x".into())), "x");
    }

    #[test]
    fn translate_identifier_uppercase() {
        assert_eq!(translate(&Expr::Identifier("A".into())), "A");
    }

    // -- Greek letters -----------------------------------------------------
    #[test]
    fn translate_greek_pi() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Pi)), "派");
    }

    #[test]
    fn translate_greek_alpha() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Alpha)), "阿尔法");
    }

    #[test]
    fn translate_greek_beta() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Beta)), "贝塔");
    }

    #[test]
    fn translate_greek_gamma() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Gamma)), "伽马");
    }

    #[test]
    fn translate_greek_delta() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Delta)), "德尔塔");
    }

    #[test]
    fn translate_greek_epsilon() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Epsilon)), "艾普西龙");
    }

    #[test]
    fn translate_greek_theta() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Theta)), "西塔");
    }

    #[test]
    fn translate_greek_lambda() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Lambda)), "兰姆达");
    }

    #[test]
    fn translate_greek_mu() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Mu)), "缪");
    }

    #[test]
    fn translate_greek_sigma() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Sigma)), "西格玛");
    }

    #[test]
    fn translate_greek_phi() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Phi)), "fai");
    }

    #[test]
    fn translate_greek_omega() {
        assert_eq!(translate(&Expr::Greek(GreekLetter::Omega)), "欧米伽");
    }

    // -- Binary arithmetic operators ---------------------------------------
    #[test]
    fn translate_add() {
        assert_eq!(
            translate(&Expr::BinaryOp(
                BinaryOpKind::Add,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "1加2"
        );
    }

    #[test]
    fn translate_sub() {
        assert_eq!(
            translate(&Expr::BinaryOp(
                BinaryOpKind::Sub,
                box_expr(Expr::Number(5.0)),
                box_expr(Expr::Number(3.0))
            )),
            "5减3"
        );
    }

    #[test]
    fn translate_mul() {
        assert_eq!(
            translate(&Expr::BinaryOp(
                BinaryOpKind::Mul,
                box_expr(Expr::Number(2.0)),
                box_expr(Expr::Number(3.0))
            )),
            "2乘3"
        );
    }

    #[test]
    fn translate_div() {
        assert_eq!(
            translate(&Expr::BinaryOp(
                BinaryOpKind::Div,
                box_expr(Expr::Number(6.0)),
                box_expr(Expr::Number(2.0))
            )),
            "6除以2"
        );
    }

    // -- Comparison operators ----------------------------------------------
    #[test]
    fn translate_eq() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Eq,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "1等于2"
        );
    }

    #[test]
    fn translate_neq() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Neq,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "1不等于2"
        );
    }

    #[test]
    fn translate_lt() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Lt,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "1小于2"
        );
    }

    #[test]
    fn translate_gt() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Gt,
                box_expr(Expr::Number(2.0)),
                box_expr(Expr::Number(1.0))
            )),
            "2大于1"
        );
    }

    #[test]
    fn translate_lte() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Lte,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "1小于等于2"
        );
    }

    #[test]
    fn translate_gte() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Gte,
                box_expr(Expr::Number(2.0)),
                box_expr(Expr::Number(1.0))
            )),
            "2大于等于1"
        );
    }

    #[test]
    fn translate_approx() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Approx,
                box_expr(Expr::Greek(GreekLetter::Pi)),
                box_expr(Expr::Number(3.14))
            )),
            "派约等于3.14"
        );
    }

    #[test]
    fn translate_napprox() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Napprox,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "1不约等于2"
        );
    }

    #[test]
    fn translate_to() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::To,
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Number(0.0))
            )),
            "x趋向于0"
        );
    }

    #[test]
    fn translate_parallel() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Parallel,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A平行于B"
        );
    }

    #[test]
    fn translate_perpendicular() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Perpendicular,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A垂直于B"
        );
    }

    #[test]
    fn translate_congruent() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Congruent,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A全等于B"
        );
    }

    #[test]
    fn translate_similar() {
        assert_eq!(
            translate(&Expr::ComparisonOp(
                ComparisonOpKind::Similar,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A相似于B"
        );
    }

    // -- Superscript (special cases) ---------------------------------------
    #[test]
    fn translate_superscript_square() {
        assert_eq!(
            translate(&Expr::Superscript(
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Number(2.0))
            )),
            "x的平方"
        );
    }

    #[test]
    fn translate_superscript_cube() {
        assert_eq!(
            translate(&Expr::Superscript(
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Number(3.0))
            )),
            "x的立方"
        );
    }

    #[test]
    fn translate_superscript_general() {
        assert_eq!(
            translate(&Expr::Superscript(
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Number(5.0))
            )),
            "x的5次方"
        );
    }

    #[test]
    fn translate_superscript_variable() {
        assert_eq!(
            translate(&Expr::Superscript(
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Identifier("n".into()))
            )),
            "x的n次方"
        );
    }

    #[test]
    fn translate_superscript_greek() {
        assert_eq!(
            translate(&Expr::Superscript(
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Greek(GreekLetter::Alpha))
            )),
            "x的阿尔法次方"
        );
    }

    // -- Subscript ---------------------------------------------------------
    #[test]
    fn translate_subscript() {
        assert_eq!(
            translate(&Expr::Subscript(
                box_expr(Expr::Identifier("a".into())),
                box_expr(Expr::Number(1.0))
            )),
            "a下标1"
        );
    }

    #[test]
    fn translate_subscript_variable() {
        assert_eq!(
            translate(&Expr::Subscript(
                box_expr(Expr::Identifier("a".into())),
                box_expr(Expr::Identifier("n".into()))
            )),
            "a下标n"
        );
    }

    // -- Functions ---------------------------------------------------------
    #[test]
    fn translate_sin() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Sin,
                box_expr(Expr::Identifier("x".into()))
            )),
            "正弦x"
        );
    }

    #[test]
    fn translate_cos() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Cos,
                box_expr(Expr::Identifier("x".into()))
            )),
            "余弦x"
        );
    }

    #[test]
    fn translate_tan() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Tan,
                box_expr(Expr::Identifier("x".into()))
            )),
            "正切x"
        );
    }

    #[test]
    fn translate_cot() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Cot,
                box_expr(Expr::Identifier("x".into()))
            )),
            "余切x"
        );
    }

    #[test]
    fn translate_sec() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Sec,
                box_expr(Expr::Identifier("x".into()))
            )),
            "正割x"
        );
    }

    #[test]
    fn translate_csc() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Csc,
                box_expr(Expr::Identifier("x".into()))
            )),
            "余割x"
        );
    }

    #[test]
    fn translate_log() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Log,
                box_expr(Expr::Identifier("x".into()))
            )),
            "对数x"
        );
    }

    #[test]
    fn translate_ln() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Ln,
                box_expr(Expr::Identifier("x".into()))
            )),
            "自然对数x"
        );
    }

    #[test]
    fn translate_lg() {
        assert_eq!(
            translate(&Expr::Function(
                MathFunction::Lg,
                box_expr(Expr::Identifier("x".into()))
            )),
            "常用对数x"
        );
    }

    // -- Fractions ---------------------------------------------------------
    #[test]
    fn translate_frac() {
        assert_eq!(
            translate(&Expr::Frac(
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )),
            "2分之1"
        );
    }

    #[test]
    fn translate_frac_variables() {
        assert_eq!(
            translate(&Expr::Frac(
                box_expr(Expr::Identifier("a".into())),
                box_expr(Expr::Identifier("b".into()))
            )),
            "b分之a"
        );
    }

    // -- Roots -------------------------------------------------------------
    #[test]
    fn translate_sqrt() {
        assert_eq!(translate(&Expr::Sqrt(box_expr(Expr::Number(2.0)))), "根号2");
    }

    #[test]
    fn translate_sqrt_n() {
        assert_eq!(
            translate(&Expr::SqrtN(
                box_expr(Expr::Number(3.0)),
                box_expr(Expr::Number(8.0))
            )),
            "3次根号8"
        );
    }

    // -- Plus/minus, minus/plus --------------------------------------------
    #[test]
    fn translate_pm() {
        assert_eq!(translate(&Expr::Pm(box_expr(Expr::Number(2.0)))), "正负2");
    }

    #[test]
    fn translate_mp() {
        // CRITICAL: mp must output "负正" (not "正负")
        assert_eq!(translate(&Expr::Mp(box_expr(Expr::Number(2.0)))), "负正2");
    }

    // -- Degree / Percent --------------------------------------------------
    #[test]
    fn translate_degree() {
        assert_eq!(
            translate(&Expr::Degree(box_expr(Expr::Number(90.0)))),
            "90度"
        );
    }

    #[test]
    fn translate_percent() {
        assert_eq!(
            translate(&Expr::Percent(box_expr(Expr::Number(50.0)))),
            "百分之50"
        );
    }

    // -- Sign prefix -------------------------------------------------------
    #[test]
    fn translate_positive_sign() {
        assert_eq!(
            translate(&Expr::Sign(SignKind::Positive, box_expr(Expr::Number(5.0)))),
            "正5"
        );
    }

    #[test]
    fn translate_negative_sign() {
        assert_eq!(
            translate(&Expr::Sign(SignKind::Negative, box_expr(Expr::Number(3.0)))),
            "负3"
        );
    }

    // -- Group (parentheses dropped) ---------------------------------------
    #[test]
    fn translate_group() {
        assert_eq!(
            translate(&Expr::Group(Box::new(Expr::BinaryOp(
                BinaryOpKind::Add,
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Number(2.0))
            )))),
            "1加2"
        );
    }

    // -- Calculus constructs -----------------------------------------------
    #[test]
    fn translate_limit() {
        assert_eq!(
            translate(&Expr::Limit(
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Number(0.0)),
                box_expr(Expr::Identifier("x".into()))
            )),
            "当x趋向于0时的极限x"
        );
    }

    #[test]
    fn translate_sum() {
        assert_eq!(
            translate(&Expr::Sum(
                box_expr(Expr::Identifier("i".into())),
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Identifier("n".into())),
                box_expr(Expr::Identifier("i".into()))
            )),
            "i从1到n求和i"
        );
    }

    #[test]
    fn translate_product() {
        assert_eq!(
            translate(&Expr::Product(
                box_expr(Expr::Identifier("i".into())),
                box_expr(Expr::Number(1.0)),
                box_expr(Expr::Identifier("n".into())),
                box_expr(Expr::Identifier("i".into()))
            )),
            "i从1到n求积i"
        );
    }

    #[test]
    fn translate_integral() {
        assert_eq!(
            translate(&Expr::Integral(
                box_expr(Expr::Identifier("a".into())),
                box_expr(Expr::Identifier("b".into())),
                box_expr(Expr::Identifier("x".into()))
            )),
            "从a到b积分x"
        );
    }

    // -- Set operators -----------------------------------------------------
    #[test]
    fn translate_set_in() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::In,
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Identifier("A".into()))
            )),
            "x属于A"
        );
    }

    #[test]
    fn translate_set_notin() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::NotIn,
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Identifier("A".into()))
            )),
            "x不属于A"
        );
    }

    #[test]
    fn translate_set_cup() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::Cup,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A并B"
        );
    }

    #[test]
    fn translate_set_cap() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::Cap,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A交B"
        );
    }

    #[test]
    fn translate_set_subset() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::Subset,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A真子集B"
        );
    }

    #[test]
    fn translate_set_superset() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::Superset,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A真超集B"
        );
    }

    #[test]
    fn translate_set_emptyset() {
        assert_eq!(
            translate(&Expr::SetOp(
                SetOpKind::Emptyset,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A空集B"
        );
    }

    // -- Logic operators ---------------------------------------------------
    #[test]
    fn translate_logic_forall() {
        assert_eq!(
            translate(&Expr::LogicOp(
                LogicOpKind::Forall,
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Identifier("P".into()))
            )),
            "x任意P"
        );
    }

    #[test]
    fn translate_logic_exists() {
        assert_eq!(
            translate(&Expr::LogicOp(
                LogicOpKind::Exists,
                box_expr(Expr::Identifier("x".into())),
                box_expr(Expr::Identifier("P".into()))
            )),
            "x存在P"
        );
    }

    #[test]
    fn translate_logic_implies() {
        assert_eq!(
            translate(&Expr::LogicOp(
                LogicOpKind::Implies,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A推出B"
        );
    }

    #[test]
    fn translate_logic_iff() {
        assert_eq!(
            translate(&Expr::LogicOp(
                LogicOpKind::Iff,
                box_expr(Expr::Identifier("A".into())),
                box_expr(Expr::Identifier("B".into()))
            )),
            "A等价于B"
        );
    }

    // -- Geometry symbols --------------------------------------------------
    #[test]
    fn translate_geometry_triangle() {
        assert_eq!(
            translate(&Expr::Geometry(GeometrySymbol::Triangle)),
            "三角形"
        );
    }

    #[test]
    fn translate_geometry_angle() {
        assert_eq!(translate(&Expr::Geometry(GeometrySymbol::Angle)), "角");
    }

    #[test]
    fn translate_geometry_parallel() {
        assert_eq!(translate(&Expr::Geometry(GeometrySymbol::Parallel)), "平行");
    }

    #[test]
    fn translate_geometry_perpendicular() {
        assert_eq!(
            translate(&Expr::Geometry(GeometrySymbol::Perpendicular)),
            "垂直"
        );
    }

    #[test]
    fn translate_geometry_congruent() {
        assert_eq!(
            translate(&Expr::Geometry(GeometrySymbol::Congruent)),
            "全等"
        );
    }

    #[test]
    fn translate_geometry_similar() {
        assert_eq!(translate(&Expr::Geometry(GeometrySymbol::Similar)), "相似");
    }

    // -- Integration tests with builder ------------------------------------
    #[test]
    fn translate_built_addition() {
        let ast = crate::builder::parse_to_ast("1 + 2").unwrap();
        assert_eq!(translate(&ast), "1加2");
    }

    #[test]
    fn translate_built_superscript_square() {
        let ast = crate::builder::parse_to_ast("x^2").unwrap();
        assert_eq!(translate(&ast), "x的平方");
    }

    #[test]
    fn translate_built_superscript_cube() {
        let ast = crate::builder::parse_to_ast("x^3").unwrap();
        assert_eq!(translate(&ast), "x的立方");
    }

    #[test]
    fn translate_built_superscript_general() {
        let ast = crate::builder::parse_to_ast("x^5").unwrap();
        assert_eq!(translate(&ast), "x的5次方");
    }

    #[test]
    fn translate_built_frac() {
        let ast = crate::builder::parse_to_ast("\\frac{1}{2}").unwrap();
        assert_eq!(translate(&ast), "2分之1");
    }

    #[test]
    fn translate_built_sqrt() {
        let ast = crate::builder::parse_to_ast("\\sqrt{2}").unwrap();
        assert_eq!(translate(&ast), "根号2");
    }

    #[test]
    fn translate_built_sqrt_n() {
        let ast = crate::builder::parse_to_ast("\\sqrt[3]{8}").unwrap();
        assert_eq!(translate(&ast), "3次根号8");
    }

    #[test]
    fn translate_built_sin() {
        let ast = crate::builder::parse_to_ast("\\sin x").unwrap();
        assert_eq!(translate(&ast), "正弦x");
    }

    #[test]
    fn translate_built_pm() {
        let ast = crate::builder::parse_to_ast("\\pm 2").unwrap();
        assert_eq!(translate(&ast), "正负2");
    }

    #[test]
    fn translate_built_mp() {
        let ast = crate::builder::parse_to_ast("\\mp 2").unwrap();
        assert_eq!(translate(&ast), "负正2");
    }

    #[test]
    fn translate_built_limit() {
        let ast = crate::builder::parse_to_ast("\\lim_{x \\to 0} x").unwrap();
        assert_eq!(translate(&ast), "当x趋向于0时的极限x");
    }

    #[test]
    fn translate_built_sum() {
        let ast = crate::builder::parse_to_ast("\\sum_{i=1}^{n} i").unwrap();
        assert_eq!(translate(&ast), "i从1到n求和i");
    }

    #[test]
    fn translate_built_product() {
        let ast = crate::builder::parse_to_ast("\\prod_{i=1}^{n} i").unwrap();
        assert_eq!(translate(&ast), "i从1到n求积i");
    }

    #[test]
    fn translate_built_integral() {
        let ast = crate::builder::parse_to_ast("\\int_{a}^{b} x").unwrap();
        assert_eq!(translate(&ast), "从a到b积分x");
    }

    #[test]
    fn translate_built_nested_frac_sqrt() {
        let ast = crate::builder::parse_to_ast("\\frac{\\sqrt{x^2+1}}{2}").unwrap();
        assert_eq!(translate(&ast), "2分之根号x的平方加1");
    }

    #[test]
    fn translate_built_pi_approx() {
        let ast = crate::builder::parse_to_ast("\\pi \\approx 3.14").unwrap();
        assert_eq!(translate(&ast), "派约等于3.14");
    }

    #[test]
    fn translate_built_comparison_parallel() {
        let ast = crate::builder::parse_to_ast("A \\parallel B").unwrap();
        assert_eq!(translate(&ast), "A平行于B");
    }

    #[test]
    fn translate_built_set_in() {
        let ast = crate::builder::parse_to_ast("x \\in A").unwrap();
        assert_eq!(translate(&ast), "x属于A");
    }

    #[test]
    fn translate_built_logic_implies() {
        let ast = crate::builder::parse_to_ast("A \\Rightarrow B").unwrap();
        assert_eq!(translate(&ast), "A推出B");
    }

    #[test]
    fn translate_built_degree() {
        let ast = crate::builder::parse_to_ast("90\\degree").unwrap();
        assert_eq!(translate(&ast), "90度");
    }

    #[test]
    fn translate_built_percent() {
        let ast = crate::builder::parse_to_ast("50%").unwrap();
        assert_eq!(translate(&ast), "百分之50");
    }

    #[test]
    fn translate_built_subscript() {
        let ast = crate::builder::parse_to_ast("a_1").unwrap();
        assert_eq!(translate(&ast), "a下标1");
    }

    #[test]
    fn translate_built_negative() {
        let ast = crate::builder::parse_to_ast("-3").unwrap();
        assert_eq!(translate(&ast), "负3");
    }

    #[test]
    fn translate_built_triangle() {
        let ast = crate::builder::parse_to_ast("\\triangle").unwrap();
        assert_eq!(translate(&ast), "三角形");
    }

    #[test]
    fn translate_built_angle() {
        let ast = crate::builder::parse_to_ast("\\angle").unwrap();
        assert_eq!(translate(&ast), "角");
    }
}
