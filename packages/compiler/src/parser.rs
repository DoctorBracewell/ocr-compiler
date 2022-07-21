use pest::{error::Error, iterators::Pairs, Parser};
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "grammar/ocr.pest"]
pub struct OCRParser;

pub fn parse(program: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    OCRParser::parse(Rule::program, program)
}

pub fn parse_with_custom_rule(program: &str, rule: Rule) -> Result<Pairs<Rule>, Error<Rule>> {
    OCRParser::parse(rule, program)
}

#[wasm_bindgen]
pub fn print_parse(program: &str) -> String {
    format!("{:?}", OCRParser::parse(Rule::program, program))
}
