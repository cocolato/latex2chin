use latex2chin::latex_parser::{ LatexParser, Rule };
use pest::{Parser, iterators::Pair};

fn parse_pair(pair: Pair<'_, Rule>) -> String {
    return match pair.as_rule() {
        Rule::latex_string => parse_pair(pair.into_inner().next().unwrap()),
        Rule::expr => pair.into_inner().map(
            |atom| {
                parse_pair(atom)
            }
        ).collect::<Vec<String>>().join(" "),
        Rule::atom => pair.to_string(),
        _ => pair.to_string(),
    }
}


fn main() {

    let latex_string = "1.2 + 3";

    let parse_res = LatexParser::parse(Rule::latex_string, latex_string).expect("parse_error").next().unwrap();

    let r = parse_pair(parse_res);

    println!("{:?}", r);
    
}

