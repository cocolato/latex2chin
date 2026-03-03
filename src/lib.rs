use latex_parser::{ LatexParser, Rule };
use pest::{Parser, iterators::Pair};
use pyo3::prelude::*;

pub mod latex_parser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn transform(pair: Pair<'_, Rule>) -> String {
    match pair.as_rule() {
        Rule::latex_string => pair.into_inner()
            .filter(|p| p.as_rule() != Rule::EOI)
            .map(|p| transform(p))
            .collect::<Vec<String>>()
            .join(" "),
        Rule::expr => pair.into_inner()
            .map(|p| transform(p))
            .collect::<Vec<String>>()
            .join(" "),
        Rule::signed_expr => {
            let mut pair_iter = pair.into_inner();
            if pair_iter.len() == 1 {
                return transform(pair_iter.next().unwrap());
            }
            let mut signs = Vec::new();
            let mut last = None;
            for p in pair_iter {
                if p.as_rule() == Rule::ADD || p.as_rule() == Rule::SUB {
                    signs.push(p);
                } else {
                    last = Some(p);
                }
            }
            let sign_str: String = signs.iter().map(|s| match s.as_rule() {
                Rule::ADD => "正",
                Rule::SUB => "负",
                _ => unreachable!(),
            }).collect();
            format!("{}{}", sign_str, transform(last.unwrap()))
        },
        Rule::atom => pair.into_inner()
            .map(|p| transform(p))
            .collect::<Vec<String>>()
            .join(" "),
        Rule::fraction => {
            let mut pair_iter = pair.into_inner();

            let v1 = pair_iter.next().unwrap();
            let v2 = pair_iter.next().unwrap();

            format!("{}分之{}", transform(v2), transform(v1))
        },
        Rule::percentage => format!("百分之{}", transform(pair.into_inner().next().unwrap())),

        // Numbers and constants
        Rule::NUMBER => pair.as_str().to_string(),
        Rule::PI => "PI".to_string(),

        // Operators
        Rule::ADD => "加".to_string(),
        Rule::SUB => "减".to_string(),
        Rule::MUL_SYMBOL => "乘".to_string(),
        Rule::DIV_SYMBOL => "除以".to_string(),

        // Comparison operators
        Rule::EQUAL => "等于".to_string(),
        Rule::NOT_EQUAL => "不等于".to_string(),
        Rule::LT => "小于".to_string(),
        Rule::GT => "大于".to_string(),
        Rule::LTE => "小于等于".to_string(),
        Rule::GTE => "大于等于".to_string(),
        Rule::APPROX => "约等于".to_string(),
        Rule::NOT_APPROX => "不约等于".to_string(),
        _ => pair.to_string(),
    }
}

#[pyfunction]
pub fn parse_latex(latex: String) -> String {

    let parse_res = LatexParser::parse(Rule::latex_string, &latex).expect("parse_error");

    parse_res.map(|pair| transform(pair)).collect::<Vec<String>>().join("\n")

}

#[pymodule]
fn latex2chin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_latex, m)?)?;
    Ok(())
}
