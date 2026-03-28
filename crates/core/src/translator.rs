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
        Expr::Sqrt(radicand) => format!("{}的平方根", translate(radicand)),
        Expr::SqrtN(degree, radicand) => match degree.as_ref() {
            Expr::Number(n) if *n == 3.0 => format!("{}的立方根", translate(radicand)),
            _ => format!("{}的{}次方根", translate(radicand), translate(degree)),
        },

        // -- Superscript (special cases for square and cube) ----------------
        Expr::Superscript(base, exp) => match exp.as_ref() {
            Expr::Number(n) if *n == 2.0 => format!("{}的平方", translate(base)),
            Expr::Number(n) if *n == 3.0 => format!("{}的立方", translate(base)),
            _ => format!("{}的{}次方", translate(base), translate(exp)),
        },

        // -- Subscript ------------------------------------------------------
        Expr::Subscript(base, sub) => {
            format!("{}{}", translate(base), translate(sub))
        }

        // -- Functions (Chinese names) --------------------------------------
        Expr::Function(func, arg) => {
            format!("{}{}", translate_function(func), translate(arg))
        }

        // -- Log with base name (special: lg = 以10为底的对数) ------------------
        // (handled inside translate_function)

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
                "{}趋近于{}时{}的极限",
                translate(var),
                translate(target),
                translate(body)
            )
        }
        Expr::Sum(var, lower, upper, body) => {
            format!(
                "对{}从{}到{}的{}求和",
                translate(var),
                translate(lower),
                translate(upper),
                translate(body)
            )
        }
        Expr::Product(var, lower, upper, _body) => {
            format!(
                "对{}从{}到{}求积",
                translate(var),
                translate(lower),
                translate(upper)
            )
        }
        Expr::Integral(lower, upper, body) => {
            format!(
                "从{}到{}的{}的定积分",
                translate(lower),
                translate(upper),
                translate(body)
            )
        }

        // -- Set operators --------------------------------------------------
        Expr::SetOp(kind, lhs, rhs) => match kind {
            SetOpKind::Subset => format!("{}是{}的子集", translate(lhs), translate(rhs)),
            SetOpKind::Superset => format!("{}是{}的超集", translate(lhs), translate(rhs)),
            _ => format!(
                "{}{}{}",
                translate(lhs),
                translate_set_op(kind),
                translate(rhs),
            ),
        },

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

        // -- Empty set constant ---------------------------------------------
        Expr::Emptyset => "空集".to_string(),
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
        MathFunction::Sin => "sin",
        MathFunction::Cos => "cos",
        MathFunction::Tan => "tan",
        MathFunction::Cot => "cot",
        MathFunction::Sec => "sec",
        MathFunction::Csc => "csc",
        MathFunction::Log => "log",
        MathFunction::Ln => "ln",
        MathFunction::Lg => "以10为底的对数",
    }
}

/// Translate a set theory operator to Chinese.
fn translate_set_op(kind: &SetOpKind) -> &'static str {
    match kind {
        SetOpKind::In => "属于",
        SetOpKind::NotIn => "不属于",
        SetOpKind::Cup => "并",
        SetOpKind::Cap => "交",
        SetOpKind::Subset => "是",
        SetOpKind::Superset => "是",
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
