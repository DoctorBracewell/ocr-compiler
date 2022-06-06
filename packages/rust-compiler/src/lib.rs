extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ocr.pest"]
pub struct OCRParser;

pub fn parse(program: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    OCRParser::parse(Rule::program, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_padded_linebreaks() {
        parse("\n\n\n// This is a comment statement!\n\n\n").unwrap();
    }

    #[test]
    fn parse_comment() {
        parse("// This is a comment statement!").unwrap();
    }

    #[test]
    fn parse_assignment() {
        parse("// This is a comment statement!").unwrap();
        parse("// This is a comment statement!").unwrap();
    }
}
