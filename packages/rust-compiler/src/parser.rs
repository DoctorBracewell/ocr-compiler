use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/ocr.pest"]
struct OCRParser;

pub fn parse(program: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    OCRParser::parse(Rule::program, program)
}
