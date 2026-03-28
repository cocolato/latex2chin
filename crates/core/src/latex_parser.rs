use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "latex.pest"]
pub struct LatexParser;
