use super::errors::{TranspilerError::*, *};
use super::symbol_table::{SymbolTable, SymbolTableEntry};
use super::transpiler_data::*;

use crate::parser::Rule;

use pest::iterators::Pair;

#[derive(Default)]
pub struct JavascriptProgram {
    program_string: ProgramString,
    symbol_table: SymbolTable,
}

#[derive(Default)]
struct ProgramString {
    text: String,
}

impl ProgramString {
    fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn prepend_text(&mut self, text: String) {
        self.text = text + &self.text;
    }
}

impl JavascriptProgram {
    pub fn initialise_variables(&mut self) {
        let text = self
            .symbol_table
            .variables
            .values()
            .fold(String::new(), |acc, s| {
                acc + &format!("{}{}{}{}", LET, SPACE, s.minified_name, SEMICOLON)
            });

        self.program_string.prepend_text(text);
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

        // Create entry in symbol table and extract minified name
        let SymbolTableEntry { minified_name, .. } =
            self.symbol_table.get_or_add_variable(identifier);

        // Push assignment to program string
        self.program_string
            .push_text(&format!("{}{}", minified_name, ASSIGNMENT_OPERATOR));

        // Transpile expression and push to program string
        let expr = parts.next().ok_or(IteratorError)?;
        self.expression(expr)?;

        // Push semicolon to program string
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
        self.program_string.push_text(SEMICOLON);
    }

    pub fn get_program_string(self) -> String {
        self.program_string.text
    }
}
