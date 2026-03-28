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
            let base = transform(inner.next().unwrap());
            match inner.next() {
                Some(s) => match s.as_rule() {
                    Rule::degree_mark => format!("{}度", base),
                    Rule::percent_mark => format!("百分之{}", base),
                    _ => unreachable!(),
                },
                None => base,
            }
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
        Rule::pm => format!("正负{}", transform(pair.into_inner().next().unwrap())),

        Rule::number => pair.as_str().to_string(),
        Rule::pi => "PI".to_string(),

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
