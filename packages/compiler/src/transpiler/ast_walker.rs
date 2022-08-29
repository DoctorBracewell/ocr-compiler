use super::errors::{TranspilerError::*, *};
use super::symbol_table::{SymbolTable, SymbolTableEntry};
use super::transpiler_data::*;

use crate::parser::Rule;

use pest::iterators::Pair;

#[derive(Default)]
pub struct JavascriptProgram {
    pub text: String,
    symbol_table: SymbolTable,
}

impl JavascriptProgram {
    fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn prepend_text(&mut self, text: String) {
        self.text = text + &self.text;
    }

    pub fn initialise_variables(&mut self) {
        let text = self
            .symbol_table
            .variables
            .into_values()
            .fold(String::new(), |acc, s| {
                acc + &format!("{}{}{}{}", LET, SPACE, s.minified_name, SEMICOLON)
            })
            .to_string();

        self.prepend_text(text.to_string());
    }

    pub fn walk_ast(&mut self, program_rule: Pair<Rule>) -> Result<(), TranspilerError> {
        for rule in program_rule.into_inner() {
            match rule.as_rule() {
                Rule::block => self.block(rule)?,
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn block(&mut self, block: Pair<Rule>) -> Result<(), TranspilerError> {
        for statement in block.into_inner() {
            match statement.as_rule() {
                Rule::assignment => self.assignment(statement)?,
                Rule::forLoop => unimplemented!(),
                Rule::ifStatement => unimplemented!(),
                Rule::expression => unimplemented!(),
                _ => unreachable!(),
            };
        }

        Ok(())
    }

    fn assignment(&mut self, assignment: Pair<Rule>) -> Result<(), TranspilerError> {
        let assignment_type = assignment.into_inner().next().ok_or(IteratorError)?;

        match assignment_type.as_rule() {
            Rule::arrayDeclaration => unimplemented!(),
            Rule::arrayAssignment => unimplemented!(),
            Rule::taggedVariableAssignment => unimplemented!(),
            Rule::variableAssignment => self.variable_assignment(assignment_type)?,

            _ => unreachable!(),
        };

        Ok(())
    }

    fn variable_assignment(&mut self, assignment: Pair<Rule>) -> Result<(), TranspilerError> {
        // Extract data from statement
        let mut parts = assignment.into_inner();
        let identifier = parts.next().ok_or(IteratorError)?.as_str().to_string();

        // Add new variable to the symbol table
        if !self.symbol_table.variables.contains_key(&identifier) {
            self.symbol_table.add_variable(&identifier)?;
        }

        // Get minified name from symbol table and push to result
        let SymbolTableEntry { minified_name, .. } = self
            .symbol_table
            .variables
            .get(&identifier)
            .ok_or(TranspilationError)?;

        self.push_text(&format!("{}{}", minified_name, ASSIGNMENT_OPERATOR));

        // Transpile expression and push to result
        let expr = parts.next().ok_or(IteratorError)?;
        self.expression(expr)?;

        self.end_statement();

        Ok(())
    }

    fn expression(&mut self, assignment: Pair<Rule>) -> Result<(), TranspilerError> {
        let mut parts = assignment.into_inner().peekable();

        let value = {
            while matches!(
                parts.peek().ok_or(IteratorError)?.as_rule(),
                Rule::logicalNotOperator
            ) {
                JavascriptProgram::logical_not_operator();
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

    fn end_statement(&mut self) {
        self.push_text(SEMICOLON);
    }
}
