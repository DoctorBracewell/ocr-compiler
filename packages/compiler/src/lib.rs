use wasm_bindgen::prelude::*;

extern crate pest;
extern crate wasm_bindgen;
#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod transpiler;
pub mod utils;

use parser::parse;
use transpiler::transpile;

#[wasm_bindgen]
pub fn parse_and_transpile(program: &str) -> String {
    let parsed = parse(program);
    let transpiled = transpile(parsed);

    transpiled
}
