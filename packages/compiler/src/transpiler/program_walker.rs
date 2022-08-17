use std::collections::binary_heap::Iter;

use super::errors::{TranspilerError::*, *};
use super::symbol_table::{SymbolTable, SymbolTableEntry};
use super::transpiler_data::*;

use crate::parser::Rule;

use pest::iterators::Pair;

#[derive(Default, Debug)]
pub struct JavascriptProgram {
    pub text: String,
}

impl JavascriptProgram {
    pub fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn prepend_text(&mut self, text: String) {
        self.text = text + &self.text;
    }
}

pub fn program(
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
    if !symbol_table.variables.contains_key(&identifier) {
        symbol_table.add_variable(&identifier)?;
    }

    // Get minified name from symbol table and push to result
    let SymbolTableEntry { minified_name, .. } = symbol_table
        .variables
        .get(&identifier)
        .ok_or(TranspilationError)?;

    result.push_text(&format!("{}{}", minified_name, ASSIGNMENT_OPERATOR));

    // Transpile expression and push to result
    let expr = parts.next().ok_or(IteratorError)?;
    expression(expr, result, symbol_table)?;

    end_statement(result);

    Ok(())
}

fn expression(
    assignment: Pair<Rule>,
    result: &mut JavascriptProgram,
    symbol_table: &mut SymbolTable,
) -> Result<(), TranspilerError> {
    let mut parts = assignment.into_inner().peekable();

    let value = {
        while matches!(
            parts.peek().ok_or(IteratorError)?.as_rule(),
            Rule::logicalNotOperator
        ) {
            logical_not_operator();
            parts.next();
        }

        parts.next().ok_or(IteratorError)?.as_rule()
    };

    dbg!(value);

    Ok(())
}

fn logical_not_operator() {
    dbg!("not!");
}

fn end_statement(result: &mut JavascriptProgram) {
    result.push_text(SEMICOLON);
}
