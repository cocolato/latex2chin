pub mod ast;
pub mod latex_parser;

use latex_parser::{LatexParser, Rule};
use pest::{iterators::Pair, Parser};

pub fn transform(pair: Pair<'_, Rule>) -> String {
    match pair.as_rule() {
        Rule::input => pair
            .into_inner()
            .filter(|p| p.as_rule() != Rule::EOI)
            .map(|p| transform(p))
            .collect::<Vec<String>>()
            .join(""),

        Rule::expr => {
            let mut result = String::new();
            for child in pair.into_inner() {
                match child.as_rule() {
                    Rule::sign => {
                        let inner = child.into_inner().next().unwrap();
                        result.push_str(match inner.as_rule() {
                            Rule::op_add => "正",
                            Rule::op_sub => "负",
                            _ => unreachable!(),
                        });
                    }
                    _ => result.push_str(&transform(child)),
                }
            }
            result
        }

        Rule::term => {
            let mut inner = pair.into_inner();
            let mut base = transform(inner.next().unwrap());
            for s in inner {
                match s.as_rule() {
                    Rule::degree_mark => base = format!("{}度", base),
                    Rule::percent_mark => base = format!("百分之{}", base),
                    Rule::superscript => {
                        let sup_inner = s.into_inner().next().unwrap();
                        base = format!("{}的{}次方", base, transform(sup_inner));
                    }
                    Rule::subscript => {
                        let sub_inner = s.into_inner().next().unwrap();
                        base = format!("{}下标{}", base, transform(sub_inner));
                    }
                    _ => unreachable!(),
                }
            }
            base
        }

        Rule::primary | Rule::group => pair
            .into_inner()
            .map(|p| transform(p))
            .collect::<Vec<String>>()
            .join(""),

        Rule::frac => {
            let mut inner = pair.into_inner();
            let numerator = transform(inner.next().unwrap());
            let denominator = transform(inner.next().unwrap());
            format!("{}分之{}", denominator, numerator)
        }

        Rule::sqrt => format!("根号{}", transform(pair.into_inner().next().unwrap())),

        Rule::sqrt_n => {
            let mut inner = pair.into_inner();
            let degree = transform(inner.next().unwrap());
            let radicand = transform(inner.next().unwrap());
            format!("{}次根号{}", degree, radicand)
        }

        Rule::pm => format!("正负{}", transform(pair.into_inner().next().unwrap())),

        Rule::function => {
            let raw = pair.as_str();
            let arg = transform(pair.into_inner().next().unwrap());
            // cmd_func is silent, so determine function name from raw text
            let func_name = if raw.starts_with("\\sin") {
                "sin"
            } else if raw.starts_with("\\cos") {
                "cos"
            } else if raw.starts_with("\\tan") {
                "tan"
            } else if raw.starts_with("\\cot") {
                "cot"
            } else if raw.starts_with("\\sec") {
                "sec"
            } else if raw.starts_with("\\csc") {
                "csc"
            } else if raw.starts_with("\\log") {
                "log"
            } else if raw.starts_with("\\ln") {
                "ln"
            } else if raw.starts_with("\\lg") {
                "lg"
            } else {
                "fn"
            };
            format!("{}{}", func_name, arg)
        }

        Rule::limit_expr => {
            let mut inner = pair.into_inner();
            let subscript = inner.next().unwrap();
            let body = transform(inner.next().unwrap());
            // subscript_bounds contains the "x -> target" expression
            let sub_expr = transform(subscript);
            format!("极限{}{}", sub_expr, body)
        }

        Rule::sum_expr => {
            let mut inner = pair.into_inner();
            let sub = transform(inner.next().unwrap());
            let sup = transform(inner.next().unwrap());
            let body = transform(inner.next().unwrap());
            format!("求和{}到{}的{}", sub, sup, body)
        }

        Rule::product_expr => {
            let mut inner = pair.into_inner();
            let sub = transform(inner.next().unwrap());
            let sup = transform(inner.next().unwrap());
            let body = transform(inner.next().unwrap());
            format!("求积{}到{}的{}", sub, sup, body)
        }

        Rule::integral_expr => {
            let mut inner = pair.into_inner();
            let sub = transform(inner.next().unwrap());
            let sup = transform(inner.next().unwrap());
            let body = transform(inner.next().unwrap());
            format!("积分从{}到{}的{}", sub, sup, body)
        }

        Rule::subscript_bounds | Rule::superscript_bounds => pair
            .into_inner()
            .map(|p| transform(p))
            .collect::<Vec<String>>()
            .join(""),

        Rule::emptyset => "空集".to_string(),

        Rule::geometry => pair.as_str().to_string(),

        Rule::number => pair.as_str().to_string(),
        Rule::identifier => pair.as_str().to_string(),
        Rule::greek => match pair.as_str() {
            "\\pi" => "PI".to_string(),
            other => other.to_string(),
        },

        Rule::op_add => "加".to_string(),
        Rule::op_sub => "减".to_string(),
        Rule::op_mul => "乘".to_string(),
        Rule::op_div => "除以".to_string(),

        Rule::op_eq => "等于".to_string(),
        Rule::op_neq => "不等于".to_string(),
        Rule::op_lt => "小于".to_string(),
        Rule::op_gt => "大于".to_string(),
        Rule::op_lte => "小于等于".to_string(),
        Rule::op_gte => "大于等于".to_string(),
        Rule::op_approx => "约等于".to_string(),
        Rule::op_napprox => "不约等于".to_string(),

        Rule::op_to => "趋于".to_string(),

        Rule::op_in => "属于".to_string(),
        Rule::op_notin => "不属于".to_string(),
        Rule::op_cup => "并".to_string(),
        Rule::op_cap => "交".to_string(),
        Rule::op_subset => "真子集".to_string(),
        Rule::op_superset => "真超集".to_string(),

        Rule::op_forall => "任意".to_string(),
        Rule::op_exists => "存在".to_string(),
        Rule::op_implies => "推出".to_string(),
        Rule::op_iff => "等价于".to_string(),

        Rule::op_parallel => "平行于".to_string(),
        Rule::op_perp => "垂直于".to_string(),
        Rule::op_congruent => "全等于".to_string(),
        Rule::op_similar => "相似于".to_string(),

        _ => pair.as_str().to_string(),
    }
}

pub fn parse_latex(latex: &str) -> String {
    let parse_res = LatexParser::parse(Rule::input, latex).expect("parse_error");
    parse_res
        .map(|pair| transform(pair))
        .collect::<Vec<String>>()
        .join("\n")
}
