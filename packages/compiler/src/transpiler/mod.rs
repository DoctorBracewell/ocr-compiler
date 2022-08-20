// Imports and modules
use crate::parser::Rule;
use pest::iterators::Pairs;

mod errors;
use errors::{TranspilerError::*, *};

mod program_walker;
use program_walker::{program, JavascriptProgram};

mod transpiler_data;
use transpiler_data::*;

mod symbol_table;
use symbol_table::*;

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
    let mut result = JavascriptProgram::default();
    let mut symbol_table = SymbolTable::default();

    // Traverse program rules to generate program
    program(program_rule, &mut result, &mut symbol_table)?;

    // Initiliase variables
    result.prepend_text(
        symbol_table
            .variables
            .into_values()
            .fold(String::new(), |acc, s| {
                acc + &format!("{}{}{}{}", LET, SPACE, s.minified_name, SEMICOLON)
            }),
    );

    Ok(result.text)
}
