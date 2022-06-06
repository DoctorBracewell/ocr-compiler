#[cfg(test)]
mod tests;

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
