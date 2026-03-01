use latex2chin::latex_parser::{ LatexParser, Rule };
use pest::{Parser};
// use std::collections::HashMap;
// use std::fs;

fn main() {

    let latex_string = "1.2 + 3";

    let parse_res = LatexParser::parse(Rule::latex_string, latex_string).expect("parse_error").tokens();

    for token in parse_res {
        println!("{token:?}");
    }

}

