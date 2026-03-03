use latex2chin::parse_latex;


fn main() {

    let latex_string = "\\frac{1}{20%}";

    let r = parse_latex(latex_string.to_string());

    println!("{:?}", r);
    
}

