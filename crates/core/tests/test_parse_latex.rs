use latex2chin_core::latex_parser::{LatexParser, Rule};
use latex2chin_core::parse_latex;
use pest::Parser;

fn p(input: &str) -> String {
    parse_latex(input)
}

fn assert_parse_fail(input: &str) {
    assert!(
        LatexParser::parse(Rule::input, input).is_err(),
        "expected parse failure for: {input}"
    );
}

// -- 1. Plain numbers -----------------------------------------
#[test]
fn number_integer() {
    assert_eq!(p("1"), "1");
    assert_eq!(p("42"), "42");
    assert_eq!(p("0"), "0");
}

#[test]
fn number_decimal() {
    assert_eq!(p("3.14"), "3.14");
    assert_eq!(p("0.5"), "0.5");
    assert_eq!(p("100.00"), "100.00");
}

// -- 2. Constants --------------------------------------------
#[test]
fn pi_constant() {
    assert_eq!(p("\\pi"), "PI");
}

// -- 3. Signs (signed_expr) ----------------------------------
#[test]
fn positive_number() {
    assert_eq!(p("+5"), "正5");
}

#[test]
fn negative_number() {
    assert_eq!(p("-3"), "负3");
}

#[test]
fn negative_decimal() {
    assert_eq!(p("-1.2"), "负1.2");
}

// -- 4. Arithmetic operations --------------------------------
#[test]
fn addition() {
    assert_eq!(p("1 + 2"), "1加2");
}

#[test]
fn subtraction() {
    assert_eq!(p("5 - 3"), "5减3");
}

#[test]
fn multiplication_asterisk() {
    assert_eq!(p("2 * 3"), "2乘3");
}

#[test]
fn multiplication_times() {
    assert_eq!(p("2 \\times 3"), "2乘3");
}

#[test]
fn multiplication_cdot() {
    assert_eq!(p("2 \\cdot 3"), "2乘3");
}

#[test]
fn division_slash() {
    assert_eq!(p("6 / 2"), "6除以2");
}

#[test]
fn division_cmd() {
    assert_eq!(p("6 \\div 2"), "6除以2");
}

// -- 5. Chained expressions ----------------------------------
#[test]
fn chained_operations() {
    assert_eq!(p("1 + 2 * 3"), "1加2乘3");
}

#[test]
fn chained_add_sub() {
    assert_eq!(p("1 + 2 - 3"), "1加2减3");
}

#[test]
fn long_chain() {
    assert_eq!(p("1 + 2 * 3 - 4 / 5"), "1加2乘3减4除以5");
}

// -- 6. Parenthesized expressions ----------------------------
#[test]
fn parenthesized_expr() {
    assert_eq!(p("(1 + 2)"), "1加2");
}

#[test]
fn paren_with_outer_op() {
    assert_eq!(p("(1 + 2) * 3"), "1加2乘3");
}

#[test]
fn nested_parens() {
    assert_eq!(p("((1 + 2))"), "1加2");
}

#[test]
fn negative_in_paren() {
    assert_eq!(p("(-1 + 2)"), "负1加2");
}

#[test]
fn paren_both_sides() {
    assert_eq!(p("(1 + 2) + (3 + 4)"), "1加2加3加4");
}

#[test]
fn complex_paren_expr() {
    assert_eq!(p("(-1.2 + 3) + 1 * 2"), "负1.2加3加1乘2");
}

// -- 7. Fractions --------------------------------------------
#[test]
fn simple_fraction() {
    assert_eq!(p("\\frac{1}{2}"), "2分之1");
}

#[test]
fn fraction_dfrac() {
    assert_eq!(p("\\dfrac{3}{4}"), "4分之3");
}

#[test]
fn fraction_tfrac() {
    assert_eq!(p("\\tfrac{5}{6}"), "6分之5");
}

#[test]
fn fraction_nicefrac() {
    assert_eq!(p("\\nicefrac{7}{8}"), "8分之7");
}

#[test]
fn fraction_with_negative() {
    assert_eq!(p("\\frac{-1}{2}"), "2分之负1");
}

#[test]
fn fraction_in_expr() {
    assert_eq!(p("\\frac{1}{2} + 3"), "2分之1加3");
}

// -- 8. Percentages ------------------------------------------
#[test]
fn percentage_simple() {
    assert_eq!(p("50%"), "百分之50");
}

#[test]
fn percentage_decimal() {
    assert_eq!(p("3.5%"), "百分之3.5");
}

#[test]
fn percentage_in_fraction() {
    assert_eq!(p("\\frac{1}{20%}"), "百分之20分之1");
}

// -- 9. Comparison operators ---------------------------------
#[test]
fn equal() {
    assert_eq!(p("1 = 2"), "1等于2");
}

#[test]
fn not_equal_neq() {
    assert_eq!(p("1 \\neq 2"), "1不等于2");
}

#[test]
fn not_equal_ne() {
    assert_eq!(p("1 \\ne 2"), "1不等于2");
}

#[test]
fn less_than() {
    assert_eq!(p("1 < 2"), "1小于2");
}

#[test]
fn less_than_cmd() {
    assert_eq!(p("1 \\lt 2"), "1小于2");
}

#[test]
fn greater_than() {
    assert_eq!(p("2 > 1"), "2大于1");
}

#[test]
fn greater_than_cmd() {
    assert_eq!(p("2 \\gt 1"), "2大于1");
}

#[test]
fn less_than_equal_le() {
    assert_eq!(p("1 \\le 2"), "1小于等于2");
}

#[test]
fn less_than_equal_leq() {
    assert_eq!(p("1 \\leq 2"), "1小于等于2");
}

#[test]
fn greater_than_equal_ge() {
    assert_eq!(p("2 \\ge 1"), "2大于等于1");
}

#[test]
fn greater_than_equal_geq() {
    assert_eq!(p("2 \\geq 1"), "2大于等于1");
}

#[test]
fn approx_cmd() {
    assert_eq!(p("\\pi \\approx 3.14"), "PI约等于3.14");
}

#[test]
fn approx_unicode() {
    assert_eq!(p("\\pi ≈ 3.14"), "PI约等于3.14");
}

// -- 10. Parse failures (reject invalid input) ---------------
#[test]
fn reject_empty() {
    assert_parse_fail("");
}

#[test]
fn reject_bare_operator() {
    assert_parse_fail("+");
}

#[test]
fn reject_unmatched_paren() {
    assert_parse_fail("(1 + 2");
}

#[test]
fn reject_trailing_operator() {
    assert_parse_fail("1 +");
}

// -- 11. Whitespace handling ---------------------------------
#[test]
fn whitespace_variations() {
    assert_eq!(p("1+2"), p("1 + 2"));
    assert_eq!(p("1  +  2"), p("1 + 2"));
}

#[test]
fn tab_and_newline() {
    assert_eq!(p("1\t+\n2"), "1加2");
}

// -- 12. Complex expressions ---------------------------------
#[test]
fn fraction_with_percentage_denominator() {
    assert_eq!(p("\\frac{1}{20%}"), "百分之20分之1");
}

#[test]
fn fraction_plus_number() {
    assert_eq!(p("\\frac{1}{3} + \\frac{2}{3}"), "3分之1加3分之2");
}

#[test]
fn negative_fraction() {
    assert_eq!(p("-\\frac{1}{2}"), "负2分之1");
}

#[test]
fn pi_in_expression() {
    assert_eq!(p("2 * \\pi"), "2乘PI");
}

#[test]
fn complex_mixed_expr() {
    assert_eq!(p("\\frac{1}{2} + 3 * (4 - 5)"), "2分之1加3乘4减5");
}

#[test]
fn pm_expression() {
    assert_eq!(p("\\pm2"), "正负2");
    assert_eq!(p("\\mp\\frac{1}{2}"), "正负2分之1");
    assert_eq!(p("±1+2"), "正负1加2");
}

#[test]
fn sqrt_expression() {
    assert_eq!(p("\\sqrt{2}"), "根号2");
}

#[test]
fn degree_expression() {
    assert_eq!(p("100\\degree"), "100度");
}

// =============================================================================
// NEW GRAMMAR TESTS - P0 and P1 syntax
// =============================================================================

// -- 13. Identifiers -----------------------------------------
#[test]
fn identifier_single_letter() {
    assert_eq!(p("x"), "x");
    assert_eq!(p("A"), "A");
    assert_eq!(p("n"), "n");
}

#[test]
fn identifier_in_expression() {
    assert_eq!(p("x + 1"), "x加1");
    assert_eq!(p("2 * n"), "2乘n");
}

// -- 14. Superscripts ----------------------------------------
#[test]
fn superscript_simple() {
    assert_eq!(p("x^2"), "x的2次方");
}

#[test]
fn superscript_braced() {
    assert_eq!(p("x^{n+1}"), "x的n加1次方");
}

#[test]
fn superscript_greek() {
    assert_eq!(p("x^\\pi"), "x的PI次方");
}

#[test]
fn superscript_in_expression() {
    assert_eq!(p("x^2 + 1"), "x的2次方加1");
}

#[test]
fn superscript_with_group() {
    assert_eq!(p("(x + 1)^2"), "x加1的2次方");
}

// -- 15. Subscripts ------------------------------------------
#[test]
fn subscript_simple() {
    assert_eq!(p("a_1"), "a下标1");
}

#[test]
fn subscript_braced() {
    assert_eq!(p("a_{n+1}"), "a下标n加1");
}

#[test]
fn subscript_identifier() {
    assert_eq!(p("x_n"), "x下标n");
}

#[test]
fn subscript_in_expression() {
    assert_eq!(p("a_1 + a_2"), "a下标1加a下标2");
}

// -- 16. Trig functions --------------------------------------
#[test]
fn sin_function() {
    assert_eq!(p("\\sin x"), "sinx");
}

#[test]
fn cos_function() {
    assert_eq!(p("\\cos x"), "cosx");
}

#[test]
fn tan_function() {
    assert_eq!(p("\\tan x"), "tanx");
}

#[test]
fn sin_with_group() {
    assert_eq!(p("\\sin(x + 1)"), "sinx加1");
}

#[test]
fn sin_with_braces() {
    assert_eq!(p("\\sin{x}"), "sinx");
}

#[test]
fn sin_with_number() {
    assert_eq!(p("\\sin 5"), "sin5");
}

#[test]
fn cot_function() {
    assert_eq!(p("\\cot x"), "cotx");
}

#[test]
fn sec_function() {
    assert_eq!(p("\\sec x"), "secx");
}

#[test]
fn csc_function() {
    assert_eq!(p("\\csc x"), "cscx");
}

// -- 17. Log functions ---------------------------------------
#[test]
fn log_function() {
    assert_eq!(p("\\log x"), "logx");
}

#[test]
fn ln_function() {
    assert_eq!(p("\\ln x"), "lnx");
}

#[test]
fn lg_function() {
    assert_eq!(p("\\lg x"), "lgx");
}

#[test]
fn log_with_braces() {
    assert_eq!(p("\\log{x}"), "logx");
}

#[test]
fn log_with_group() {
    assert_eq!(p("\\log(x + 1)"), "logx加1");
}

// -- 18. Greek letters ---------------------------------------
#[test]
fn greek_alpha() {
    assert_eq!(p("\\alpha"), "\\alpha");
}

#[test]
fn greek_beta() {
    assert_eq!(p("\\beta"), "\\beta");
}

#[test]
fn greek_gamma() {
    assert_eq!(p("\\gamma"), "\\gamma");
}

#[test]
fn greek_delta() {
    assert_eq!(p("\\delta"), "\\delta");
}

#[test]
fn greek_epsilon() {
    assert_eq!(p("\\epsilon"), "\\epsilon");
}

#[test]
fn greek_theta() {
    assert_eq!(p("\\theta"), "\\theta");
}

#[test]
fn greek_lambda() {
    assert_eq!(p("\\lambda"), "\\lambda");
}

#[test]
fn greek_mu() {
    assert_eq!(p("\\mu"), "\\mu");
}

#[test]
fn greek_sigma() {
    assert_eq!(p("\\sigma"), "\\sigma");
}

#[test]
fn greek_pi() {
    assert_eq!(p("\\pi"), "PI");
}

#[test]
fn greek_phi() {
    assert_eq!(p("\\phi"), "\\phi");
}

#[test]
fn greek_omega() {
    assert_eq!(p("\\omega"), "\\omega");
}

#[test]
fn greek_in_expression() {
    assert_eq!(p("\\alpha + \\beta"), "\\alpha加\\beta");
    assert_eq!(p("2 * \\pi"), "2乘PI");
}

// -- 19. Nth root --------------------------------------------
#[test]
fn nth_root_cubic() {
    assert_eq!(p("\\sqrt[3]{8}"), "3次根号8");
}

#[test]
fn nth_root_general() {
    assert_eq!(p("\\sqrt[n]{x}"), "n次根号x");
}

#[test]
fn nth_root_in_expression() {
    assert_eq!(p("\\sqrt[3]{8} + 1"), "3次根号8加1");
}

// -- 20. Calculus: limit -------------------------------------
#[test]
fn limit_simple() {
    assert!(LatexParser::parse(Rule::input, "\\lim_{x \\to 0} x").is_ok());
}

#[test]
fn limit_with_expr() {
    assert!(LatexParser::parse(Rule::input, "\\lim_{x \\to 0} x + 1").is_ok());
}

#[test]
fn limit_in_expression() {
    assert!(LatexParser::parse(Rule::input, "\\lim_{x \\to 0} x + 1").is_ok());
}

// -- 21. Calculus: sum ---------------------------------------
#[test]
fn sum_simple() {
    assert!(LatexParser::parse(Rule::input, "\\sum_{i=1}^{n} i").is_ok());
}

#[test]
fn sum_with_frac() {
    assert!(LatexParser::parse(Rule::input, "\\sum_{i=1}^{n} \\frac{1}{i}").is_ok());
}

// -- 22. Calculus: product -----------------------------------
#[test]
fn product_simple() {
    assert!(LatexParser::parse(Rule::input, "\\prod_{i=1}^{n} i").is_ok());
}

// -- 23. Calculus: integral ----------------------------------
#[test]
fn integral_simple() {
    assert!(LatexParser::parse(Rule::input, "\\int_{0}^{1} x").is_ok());
}

#[test]
fn integral_with_expression() {
    assert!(LatexParser::parse(Rule::input, "\\int_{a}^{b} x + 1").is_ok());
}

// -- 24. Set theory operators --------------------------------
#[test]
fn set_in() {
    assert_eq!(p("x \\in A"), "x属于A");
}

#[test]
fn set_notin() {
    assert_eq!(p("x \\notin A"), "x不属于A");
}

#[test]
fn set_cup() {
    assert_eq!(p("A \\cup B"), "A并B");
}

#[test]
fn set_cap() {
    assert_eq!(p("A \\cap B"), "A交B");
}

#[test]
fn set_subset() {
    assert_eq!(p("A \\subset B"), "A真子集B");
}

#[test]
fn set_superset() {
    assert_eq!(p("A \\supset B"), "A真超集B");
}

#[test]
fn set_emptyset() {
    assert_eq!(p("\\emptyset"), "空集");
}

#[test]
fn set_emptyset_varnothing() {
    assert_eq!(p("\\varnothing"), "空集");
}

// -- 25. Logic operators -------------------------------------
#[test]
fn logic_forall() {
    assert_eq!(p("P \\forall x"), "P任意x");
}

#[test]
fn logic_exists() {
    assert_eq!(p("P \\exists x"), "P存在x");
}

#[test]
fn logic_implies() {
    assert_eq!(p("A \\Rightarrow B"), "A推出B");
}

#[test]
fn logic_implies_alt() {
    assert_eq!(p("A \\implies B"), "A推出B");
}

#[test]
fn logic_iff() {
    assert_eq!(p("A \\iff B"), "A等价于B");
}

#[test]
fn logic_iff_alt() {
    assert_eq!(p("A \\Leftrightarrow B"), "A等价于B");
}

// -- 26. Geometry operators ----------------------------------
#[test]
fn geo_parallel() {
    assert_eq!(p("A \\parallel B"), "A平行于B");
}

#[test]
fn geo_perp() {
    assert_eq!(p("A \\perp B"), "A垂直于B");
}

#[test]
fn geo_congruent() {
    assert_eq!(p("A \\cong B"), "A全等于B");
}

#[test]
fn geo_similar() {
    assert_eq!(p("A \\sim B"), "A相似于B");
}

// -- 27. Geometry standalone symbols -------------------------
#[test]
fn geo_triangle() {
    assert_eq!(p("\\triangle"), "\\triangle");
}

#[test]
fn geo_angle() {
    assert_eq!(p("\\angle"), "\\angle");
}

// -- 28. Arrow operator (to) --------------------------------
#[test]
fn op_to() {
    assert_eq!(p("x \\to 0"), "x趋于0");
}

#[test]
fn op_to_arrow() {
    assert_eq!(p("x \\rightarrow 0"), "x趋于0");
}

// -- 29. Combined constructs ---------------------------------
#[test]
fn superscript_and_subscript() {
    assert!(LatexParser::parse(Rule::input, "x_1^2").is_ok());
}

#[test]
fn subscript_then_superscript() {
    assert!(LatexParser::parse(Rule::input, "a_n^{n+1}").is_ok());
}

#[test]
fn function_in_expression() {
    assert_eq!(p("\\sin x + \\cos x"), "sinx加cosx");
}

#[test]
fn nested_function() {
    assert!(LatexParser::parse(Rule::input, "\\sin \\cos x").is_ok());
}

#[test]
fn greek_with_superscript() {
    assert!(LatexParser::parse(Rule::input, "\\alpha^2").is_ok());
}

#[test]
fn frac_with_superscript() {
    assert!(LatexParser::parse(Rule::input, "\\frac{1}{2}^3").is_ok());
}

// -- 30. Reject invalid new syntax ---------------------------
#[test]
fn reject_bare_caret() {
    assert_parse_fail("^");
}

#[test]
fn reject_bare_underscore() {
    assert_parse_fail("_");
}

#[test]
fn reject_caret_nothing() {
    assert_parse_fail("x^");
}

#[test]
fn reject_underscore_nothing() {
    assert_parse_fail("x_");
}

#[test]
fn reject_sqrt_no_brace() {
    assert_parse_fail("\\sqrt");
}

#[test]
fn reject_nth_root_no_brace() {
    assert_parse_fail("\\sqrt[3]");
}

#[test]
fn reject_function_no_arg() {
    assert_parse_fail("\\sin");
}

#[test]
fn reject_lim_no_subscript() {
    assert_parse_fail("\\lim x");
}

#[test]
fn reject_sum_no_bounds() {
    assert_parse_fail("\\sum x");
}
