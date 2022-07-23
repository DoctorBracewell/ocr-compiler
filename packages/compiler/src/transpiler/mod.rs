// Imports and modules
use std::collections::HashMap;

use crate::{parser::Rule, utils::denary_to_alphabet};
use pest::iterators::{Pair, Pairs};

mod errors;
use errors::{TranspilerError::*, *};

mod transpiler_data;
use transpiler_data::*;

// Symbol table entry, one per identifier
#[derive(Debug, Clone)]
struct SymbolTableEntry {
    minified_name: String,
    constant: bool,
}

fn add_table_entry(table: &mut SymbolTable, identifier: &str) -> Result<(), TranspilerError> {
    let identifier_copy = identifier.to_string();

    table.insert(
        identifier_copy,
        SymbolTableEntry {
            minified_name: denary_to_alphabet(table.len().try_into().unwrap()),
            constant: false,
        },
    );

    Ok(())
}

type SymbolTable = HashMap<String, SymbolTableEntry>;

#[derive(Default, Debug)]
struct JavascriptProgram {
    text: String,
}

impl JavascriptProgram {
    fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn prepend_text(&mut self, text: String) {
        self.text = text + &self.text;
    }
}

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
    let mut symbol_table: SymbolTable = HashMap::default();

    // Traverse program rules to generate program
    program(program_rule, &mut result, &mut symbol_table)?;

    // Initiliase variables
    result.prepend_text(symbol_table.into_values().fold(String::new(), |acc, s| {
        acc + &format!("{}{}{}{}", LET, SPACE, s.minified_name, SEMICOLON)
    }));

    Ok(result.text)
}

fn program(
    program_rule: Pair<Rule>,
    result: &mut JavascriptProgram,
    symbol_table: &mut SymbolTable,
) -> Result<(), TranspilerError> {
    for rule in program_rule.into_inner() {
        match rule.as_rule() {
            Rule::block => block(rule, result, symbol_table)?,
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn block(
    block: Pair<Rule>,
    result: &mut JavascriptProgram,
    symbol_table: &mut SymbolTable,
) -> Result<(), TranspilerError> {
    for statement in block.into_inner() {
        match statement.as_rule() {
            Rule::assignment => assignment(statement, result, symbol_table)?,
            Rule::forLoop => unimplemented!(),
            Rule::ifStatement => unimplemented!(),
            Rule::expression => unimplemented!(),
            _ => unreachable!(),
        };
    }

    Ok(())
}

fn assignment(
    assignment: Pair<Rule>,
    result: &mut JavascriptProgram,
    symbol_table: &mut SymbolTable,
) -> Result<(), TranspilerError> {
    let assignment_type = assignment.into_inner().next().ok_or(IteratorError)?;

    match assignment_type.as_rule() {
        Rule::arrayDeclaration => unimplemented!(),
        Rule::arrayAssignment => unimplemented!(),
        Rule::taggedVariableAssignment => unimplemented!(),
        Rule::variableAssignment => variable_assignment(assignment_type, result, symbol_table)?,

        _ => unreachable!(),
    };

    Ok(())
}

fn variable_assignment(
    assignment: Pair<Rule>,
    result: &mut JavascriptProgram,
    symbol_table: &mut SymbolTable,
) -> Result<(), TranspilerError> {
    // Extract data from statement
    let mut parts = assignment.into_inner();
    let identifier = parts.next().ok_or(IteratorError)?.as_str().to_string();

    // Add new variable to the symbol table
    if !symbol_table.contains_key(&identifier) {
        add_table_entry(symbol_table, &identifier)?;
    }

    // Get minified name from symbol table and push to result
    let SymbolTableEntry { minified_name, .. } =
        symbol_table.get(&identifier).ok_or(TranspilationError)?;

    result.push_text(&format!("{}{}", minified_name, ASSIGNMENT_OPERATOR));

    // Transpile expression and push to result
    // expression(parts.next(), result);

    end_statement(result);

    Ok(())
}

// fn expression(assignment: Pair<Rule>, result: &mut JavascriptProgram) {}

fn end_statement(result: &mut JavascriptProgram) {
    result.push_text(SEMICOLON);
}
