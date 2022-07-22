use std::collections::HashMap;

use crate::{parser::Rule, utils::denary_to_alphabet};
use pest::iterators::{Pair, Pairs};

mod transpiler_data;
use transpiler_data::*;

#[derive(Default, Debug)]
struct SymbolTableEntry {
    minified_name: String,
    constant: bool,
}

#[derive(Default, Debug)]
struct SymbolTable {
    table: HashMap<String, SymbolTableEntry>,
    naming: u32,
}

impl SymbolTable {
    fn add_entry(&mut self, identifier: &str) {
        self.table.insert(
            identifier.to_string(),
            SymbolTableEntry {
                minified_name: denary_to_alphabet(self.naming),
                constant: false,
            },
        );

        self.naming += 1;
    }
}

#[derive(Default, Debug)]
struct JavascriptProgram {
    text: String,
    symbol_table: SymbolTable,
}

impl JavascriptProgram {
    fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
    }
}

pub fn transpile(mut rules: Pairs<Rule>) -> String {
    let program = rules.next().unwrap(); // Unwrap the program rule, never fails
    let mut result = JavascriptProgram::default();

    for rule in program.into_inner() {
        match rule.as_rule() {
            Rule::block => block(rule, &mut result),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    result.text
}

fn block(block: Pair<Rule>, result: &mut JavascriptProgram) {
    for statement in block.into_inner() {
        match statement.as_rule() {
            Rule::assignment => assignment(statement, result),
            Rule::forLoop => unimplemented!(),
            Rule::ifStatement => unimplemented!(),
            Rule::expression => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

fn assignment(assignment: Pair<Rule>, result: &mut JavascriptProgram) {
    let assignment_type = assignment.into_inner().next().unwrap();

    match assignment_type.as_rule() {
        Rule::arrayDeclaration => unimplemented!(),
        Rule::arrayAssignment => unimplemented!(),
        Rule::taggedVariableAssignment => unimplemented!(),
        Rule::variableAssignment => variable_assignment(assignment_type, result),

        _ => unreachable!(),
    }
}

fn variable_assignment(assignment: Pair<Rule>, result: &mut JavascriptProgram) {
    // Extract data from statement
    let mut parts = assignment.into_inner();
    let identifier = parts.next().unwrap().as_str().to_string();

    // Add new variable to the symbol table
    result.symbol_table.add_entry(&identifier);

    // Get minified name from symbol table and push to result
    let SymbolTableEntry {minified_name, ..} = result.symbol_table.table.get(&identifier).unwrap();
    result.push_text(&format!("{}{}{}{}", LET, SPACE, minified_name, ASSIGNMENT_OPERATOR));

    // Transpile expression and push to result
    // expression(parts.next(), result);

    end_statement(result);
}

fn expression(assignment: Pair<Rule>, result: &mut JavascriptProgram) {

}

fn end_statement(result: &mut JavascriptProgram) {
    result.push_text(SEMICOLON);
}