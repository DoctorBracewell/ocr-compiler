// Imports and modules
use crate::parser::Rule;
use pest::iterators::Pairs;

mod ast_walker;
mod errors;
mod symbol_table;
mod transpiler_data;

use ast_walker::JavascriptProgram;
use errors::{TranspilerError::*, *};

pub fn transpile(rules: Pairs<Rule>) -> String {
    match transpilation_stages(rules) {
        Ok(text) => text,
        Err(err) => handle_errors(err),
    }
}

fn handle_errors(error: TranspilerError) -> String {
    dbg!(error);
    String::from("error")
}

fn transpilation_stages(mut rules: Pairs<Rule>) -> Result<String, TranspilerError> {
    // Initialise program
    let program_rule = rules.next().ok_or(TranspilationError)?;
    let mut program = JavascriptProgram::default();

    // Traverse program rules to generate program
    program.walk_ast(program_rule)?;
    program.initialise_variables();

    Ok(program.get_program_string())
}
