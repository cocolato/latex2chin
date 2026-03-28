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
