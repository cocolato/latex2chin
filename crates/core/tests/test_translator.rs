use latex2chin_core::ast::*;
use latex2chin_core::translator::translate;

// Helper to box an expression.
fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

// ── Numbers ──────────────────────────────────────────────────────────────────

#[test]
fn number_integer() {
    assert_eq!(translate(&Expr::Number(42.0)), "42");
}

#[test]
fn number_decimal() {
    assert_eq!(translate(&Expr::Number(3.14)), "3.14");
}

#[test]
fn number_zero() {
    assert_eq!(translate(&Expr::Number(0.0)), "0");
}

#[test]
fn number_one() {
    assert_eq!(translate(&Expr::Number(1.0)), "1");
}

#[test]
fn number_negative_int() {
    // Negative numbers are wrapped in Sign, so Number holds the magnitude.
    assert_eq!(translate(&Expr::Number(-5.0)), "-5");
}

// ── Identifiers ──────────────────────────────────────────────────────────────

#[test]
fn identifier() {
    assert_eq!(translate(&Expr::Identifier("x".into())), "x");
}

#[test]
fn identifier_uppercase() {
    assert_eq!(translate(&Expr::Identifier("A".into())), "A");
}

// ── Greek letters ────────────────────────────────────────────────────────────

#[test]
fn greek_pi() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Pi)), "派");
}

#[test]
fn greek_alpha() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Alpha)), "阿尔法");
}

#[test]
fn greek_beta() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Beta)), "贝塔");
}

#[test]
fn greek_gamma() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Gamma)), "伽马");
}

#[test]
fn greek_delta() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Delta)), "德尔塔");
}

#[test]
fn greek_epsilon() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Epsilon)), "艾普西龙");
}

#[test]
fn greek_theta() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Theta)), "西塔");
}

#[test]
fn greek_lambda() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Lambda)), "兰姆达");
}

#[test]
fn greek_mu() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Mu)), "缪");
}

#[test]
fn greek_sigma() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Sigma)), "西格玛");
}

#[test]
fn greek_phi() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Phi)), "fai");
}

#[test]
fn greek_omega() {
    assert_eq!(translate(&Expr::Greek(GreekLetter::Omega)), "欧米伽");
}

// ── Binary arithmetic operators ──────────────────────────────────────────────

#[test]
fn add() {
    assert_eq!(
        translate(&Expr::BinaryOp(
            BinaryOpKind::Add,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )),
        "1加2"
    );
}

#[test]
fn sub() {
    assert_eq!(
        translate(&Expr::BinaryOp(
            BinaryOpKind::Sub,
            b(Expr::Number(5.0)),
            b(Expr::Number(3.0))
        )),
        "5减3"
    );
}

#[test]
fn mul() {
    assert_eq!(
        translate(&Expr::BinaryOp(
            BinaryOpKind::Mul,
            b(Expr::Number(2.0)),
            b(Expr::Number(3.0))
        )),
        "2乘3"
    );
}

#[test]
fn div() {
    assert_eq!(
        translate(&Expr::BinaryOp(
            BinaryOpKind::Div,
            b(Expr::Number(6.0)),
            b(Expr::Number(2.0))
        )),
        "6除以2"
    );
}

// ── Comparison operators ─────────────────────────────────────────────────────

#[test]
fn eq() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Eq,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )),
        "1等于2"
    );
}

#[test]
fn neq() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Neq,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )),
        "1不等于2"
    );
}

#[test]
fn lt() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Lt,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )),
        "1小于2"
    );
}

#[test]
fn gt() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Gt,
            b(Expr::Number(2.0)),
            b(Expr::Number(1.0))
        )),
        "2大于1"
    );
}

#[test]
fn lte() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Lte,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )),
        "1小于等于2"
    );
}

#[test]
fn gte() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Gte,
            b(Expr::Number(2.0)),
            b(Expr::Number(1.0))
        )),
        "2大于等于1"
    );
}

#[test]
fn approx() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Approx,
            b(Expr::Greek(GreekLetter::Pi)),
            b(Expr::Number(3.14))
        )),
        "派约等于3.14"
    );
}

#[test]
fn napprox() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Napprox,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )),
        "1不约等于2"
    );
}

#[test]
fn to() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::To,
            b(Expr::Identifier("x".into())),
            b(Expr::Number(0.0))
        )),
        "x趋向于0"
    );
}

#[test]
fn parallel() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Parallel,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A平行于B"
    );
}

#[test]
fn perpendicular() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Perpendicular,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A垂直于B"
    );
}

#[test]
fn congruent() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Congruent,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A全等于B"
    );
}

#[test]
fn similar() {
    assert_eq!(
        translate(&Expr::ComparisonOp(
            ComparisonOpKind::Similar,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A相似于B"
    );
}

// ── Superscript (special cases) ──────────────────────────────────────────────

#[test]
fn superscript_square() {
    assert_eq!(
        translate(&Expr::Superscript(
            b(Expr::Identifier("x".into())),
            b(Expr::Number(2.0))
        )),
        "x的平方"
    );
}

#[test]
fn superscript_cube() {
    assert_eq!(
        translate(&Expr::Superscript(
            b(Expr::Identifier("x".into())),
            b(Expr::Number(3.0))
        )),
        "x的立方"
    );
}

#[test]
fn superscript_general() {
    assert_eq!(
        translate(&Expr::Superscript(
            b(Expr::Identifier("x".into())),
            b(Expr::Number(5.0))
        )),
        "x的5次方"
    );
}

#[test]
fn superscript_variable() {
    assert_eq!(
        translate(&Expr::Superscript(
            b(Expr::Identifier("x".into())),
            b(Expr::Identifier("n".into()))
        )),
        "x的n次方"
    );
}

#[test]
fn superscript_greek() {
    assert_eq!(
        translate(&Expr::Superscript(
            b(Expr::Identifier("x".into())),
            b(Expr::Greek(GreekLetter::Alpha))
        )),
        "x的阿尔法次方"
    );
}

// ── Subscript ────────────────────────────────────────────────────────────────

#[test]
fn subscript_number() {
    assert_eq!(
        translate(&Expr::Subscript(
            b(Expr::Identifier("a".into())),
            b(Expr::Number(1.0))
        )),
        "a1"
    );
}

#[test]
fn subscript_variable() {
    assert_eq!(
        translate(&Expr::Subscript(
            b(Expr::Identifier("a".into())),
            b(Expr::Identifier("n".into()))
        )),
        "an"
    );
}

// ── Functions ────────────────────────────────────────────────────────────────

#[test]
fn sin() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Sin,
            b(Expr::Identifier("x".into()))
        )),
        "sinx"
    );
}

#[test]
fn cos() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Cos,
            b(Expr::Identifier("x".into()))
        )),
        "cosx"
    );
}

#[test]
fn tan() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Tan,
            b(Expr::Identifier("x".into()))
        )),
        "tanx"
    );
}

#[test]
fn cot() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Cot,
            b(Expr::Identifier("x".into()))
        )),
        "cotx"
    );
}

#[test]
fn sec() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Sec,
            b(Expr::Identifier("x".into()))
        )),
        "secx"
    );
}

#[test]
fn csc() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Csc,
            b(Expr::Identifier("x".into()))
        )),
        "cscx"
    );
}

#[test]
fn log() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Log,
            b(Expr::Identifier("x".into()))
        )),
        "logx"
    );
}

#[test]
fn ln() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Ln,
            b(Expr::Identifier("x".into()))
        )),
        "lnx"
    );
}

#[test]
fn lg() {
    assert_eq!(
        translate(&Expr::Function(
            MathFunction::Lg,
            b(Expr::Identifier("x".into()))
        )),
        "以10为底的对数x"
    );
}

// ── Fractions ────────────────────────────────────────────────────────────────

#[test]
fn frac() {
    assert_eq!(
        translate(&Expr::Frac(b(Expr::Number(1.0)), b(Expr::Number(2.0)))),
        "2分之1"
    );
}

#[test]
fn frac_variables() {
    assert_eq!(
        translate(&Expr::Frac(
            b(Expr::Identifier("a".into())),
            b(Expr::Identifier("b".into()))
        )),
        "b分之a"
    );
}

// ── Roots ────────────────────────────────────────────────────────────────────

#[test]
fn sqrt() {
    assert_eq!(translate(&Expr::Sqrt(b(Expr::Number(2.0)))), "2的平方根");
}

#[test]
fn sqrt_n() {
    assert_eq!(
        translate(&Expr::SqrtN(b(Expr::Number(3.0)), b(Expr::Number(8.0)))),
        "8的立方根"
    );
}

// ── Plus/minus, minus/plus ───────────────────────────────────────────────────

#[test]
fn pm() {
    assert_eq!(translate(&Expr::Pm(b(Expr::Number(2.0)))), "正负2");
}

#[test]
fn mp() {
    // CRITICAL: mp must output "负正" (not "正负")
    assert_eq!(translate(&Expr::Mp(b(Expr::Number(2.0)))), "负正2");
}

// ── Degree / Percent ─────────────────────────────────────────────────────────

#[test]
fn degree() {
    assert_eq!(translate(&Expr::Degree(b(Expr::Number(90.0)))), "90度");
}

#[test]
fn percent() {
    assert_eq!(translate(&Expr::Percent(b(Expr::Number(50.0)))), "百分之50");
}

// ── Sign prefix ──────────────────────────────────────────────────────────────

#[test]
fn positive_sign() {
    assert_eq!(
        translate(&Expr::Sign(SignKind::Positive, b(Expr::Number(5.0)))),
        "正5"
    );
}

#[test]
fn negative_sign() {
    assert_eq!(
        translate(&Expr::Sign(SignKind::Negative, b(Expr::Number(3.0)))),
        "负3"
    );
}

// ── Group (parentheses dropped) ──────────────────────────────────────────────

#[test]
fn group() {
    assert_eq!(
        translate(&Expr::Group(Box::new(Expr::BinaryOp(
            BinaryOpKind::Add,
            b(Expr::Number(1.0)),
            b(Expr::Number(2.0))
        )))),
        "1加2"
    );
}

// ── Calculus constructs ──────────────────────────────────────────────────────

#[test]
fn limit() {
    assert_eq!(
        translate(&Expr::Limit(
            b(Expr::Identifier("x".into())),
            b(Expr::Number(0.0)),
            b(Expr::Identifier("x".into()))
        )),
        "x趋近于0时x的极限"
    );
}

#[test]
fn sum() {
    assert_eq!(
        translate(&Expr::Sum(
            b(Expr::Identifier("i".into())),
            b(Expr::Number(1.0)),
            b(Expr::Identifier("n".into())),
            b(Expr::Identifier("i".into()))
        )),
        "对i从1到n的i求和"
    );
}

#[test]
fn product() {
    assert_eq!(
        translate(&Expr::Product(
            b(Expr::Identifier("i".into())),
            b(Expr::Number(1.0)),
            b(Expr::Identifier("n".into())),
            b(Expr::Identifier("i".into()))
        )),
        "对i从1到n求积"
    );
}

#[test]
fn integral() {
    assert_eq!(
        translate(&Expr::Integral(
            b(Expr::Identifier("a".into())),
            b(Expr::Identifier("b".into())),
            b(Expr::Identifier("x".into()))
        )),
        "从a到b的x的定积分"
    );
}

// ── Set operators ────────────────────────────────────────────────────────────

#[test]
fn set_in() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::In,
            b(Expr::Identifier("x".into())),
            b(Expr::Identifier("A".into()))
        )),
        "x属于A"
    );
}

#[test]
fn set_notin() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::NotIn,
            b(Expr::Identifier("x".into())),
            b(Expr::Identifier("A".into()))
        )),
        "x不属于A"
    );
}

#[test]
fn set_cup() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::Cup,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A并B"
    );
}

#[test]
fn set_cap() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::Cap,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A交B"
    );
}

#[test]
fn set_subset() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::Subset,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A是B的子集"
    );
}

#[test]
fn set_superset() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::Superset,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A是B的超集"
    );
}

#[test]
fn set_emptyset_op() {
    assert_eq!(
        translate(&Expr::SetOp(
            SetOpKind::Emptyset,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A空集B"
    );
}

// ── Logic operators ──────────────────────────────────────────────────────────

#[test]
fn logic_forall() {
    assert_eq!(
        translate(&Expr::LogicOp(
            LogicOpKind::Forall,
            b(Expr::Identifier("x".into())),
            b(Expr::Identifier("P".into()))
        )),
        "x任意P"
    );
}

#[test]
fn logic_exists() {
    assert_eq!(
        translate(&Expr::LogicOp(
            LogicOpKind::Exists,
            b(Expr::Identifier("x".into())),
            b(Expr::Identifier("P".into()))
        )),
        "x存在P"
    );
}

#[test]
fn logic_implies() {
    assert_eq!(
        translate(&Expr::LogicOp(
            LogicOpKind::Implies,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A推出B"
    );
}

#[test]
fn logic_iff() {
    assert_eq!(
        translate(&Expr::LogicOp(
            LogicOpKind::Iff,
            b(Expr::Identifier("A".into())),
            b(Expr::Identifier("B".into()))
        )),
        "A等价于B"
    );
}

// ── Geometry symbols ─────────────────────────────────────────────────────────

#[test]
fn geometry_triangle() {
    assert_eq!(
        translate(&Expr::Geometry(GeometrySymbol::Triangle)),
        "三角形"
    );
}

#[test]
fn geometry_angle() {
    assert_eq!(translate(&Expr::Geometry(GeometrySymbol::Angle)), "角");
}

#[test]
fn geometry_parallel() {
    assert_eq!(translate(&Expr::Geometry(GeometrySymbol::Parallel)), "平行");
}

#[test]
fn geometry_perpendicular() {
    assert_eq!(
        translate(&Expr::Geometry(GeometrySymbol::Perpendicular)),
        "垂直"
    );
}

#[test]
fn geometry_congruent() {
    assert_eq!(
        translate(&Expr::Geometry(GeometrySymbol::Congruent)),
        "全等"
    );
}

#[test]
fn geometry_similar() {
    assert_eq!(translate(&Expr::Geometry(GeometrySymbol::Similar)), "相似");
}

// ── Emptyset constant ────────────────────────────────────────────────────────

#[test]
fn emptyset() {
    assert_eq!(translate(&Expr::Emptyset), "空集");
}
