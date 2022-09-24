use std::iter::Peekable;

use super::errors::{TranspilerError::*, *};
use super::scopes::{Scope, VariableTableEntry};
use super::transpiler_data::*;

use crate::parser::Rule;

use pest::iterators::{Pair, Pairs};

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Boolean,
    Number,
    Void,
    File,
}

impl Default for Type {
    fn default() -> Self {
        Type::Void
    }
}

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
        let mut scope = Scope::default();

        for rule in program_rule.into_inner() {
            match rule.as_rule() {
                Rule::block => self.block(rule, scope)?,
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn block(&mut self, block: Pair<Rule>, scope: Scope) -> Result<(), TranspilerError> {
        for statement in block.into_inner() {
            match statement.as_rule() {
                Rule::assignment => self.assignment(statement, scope)?,
                // Rule::forLoop => unimplemented!(),
                // Rule::ifStatement => unimplemented!(),
                // Rule::expression => self.expression(statement).map(|_| ())?,
                _ => unreachable!(),
            };

            // Push semicolon to program string
            self.end_statement();
        }

        Ok(())
    }

    fn assignment(&mut self, assignment: Pair<Rule>, scope: Scope) -> Result<(), TranspilerError> {
        let assignment_type = assignment.into_inner().next().ok_or(IteratorError)?;

        match assignment_type.as_rule() {
            // Rule::arrayDeclaration => unimplemented!(),
            // Rule::arrayAssignment => unimplemented!(),
            // Rule::taggedVariableAssignment => unimplemented!(),
            Rule::variableAssignment => self.variable_assignment(assignment_type, scope)?,

            _ => unreachable!(),
        };

        Ok(())
    }

    fn variable_assignment(
        &mut self,
        assignment: Pair<Rule>,
        scope: Scope,
    ) -> Result<(), TranspilerError> {
        // Extract data from statement
        let mut parts = assignment.into_inner();
        let identifier = parts.next().ok_or(IteratorError)?;

        // Create entry in symbol table and extract minified name
        let VariableTableEntry { minified_name, .. } = self
            .symbol_table
            .insert_or_update_variable(&identifier, Type::Void);

        // Push assignment to program string
        self.program_string
            .push_text(&format!("{}{}", minified_name, ASSIGNMENT_OPERATOR));

        // Transpile expression and push to program string
        let expr = parts.next().ok_or(IteratorError)?;
        let data_type = self.expression(expr)?;

        self.symbol_table
            .set_variable_type_or(&identifier, data_type, TranspilationError)?;

        Ok(())
    }

    fn expression(&mut self, expression: Pair<Rule>) -> Result<Type, TranspilerError> {
        let expr = &expression.as_str();
        let mut parts = expression.into_inner().peekable();
        let mut data_type = Type::Void;

        self.program_string.push_text(OPEN_PARENTHESIS);

        while parts.peek().is_some() {
            data_type = self.term(&mut parts, expr)?;

            if let Some(operator) = parts.next() {
                data_type = self.operator(operator)?;
            }
        }

        self.program_string.push_text(CLOSE_PARENTHESIS);

        Ok(data_type)
    }

    fn term(
        &mut self,
        parts: &mut Peekable<Pairs<Rule>>,
        expression: &str,
    ) -> Result<Type, TranspilerError> {
        print!("yes");
        let mut acc = 0;

        while let Some(part) = parts.peek() {
            if part.as_rule() == Rule::logicalNotOperator {
                parts.next();
                acc += 1;
            }

            if parts.peek().ok_or(IteratorError)?.as_rule() != Rule::logicalNotOperator {
                break;
            }
        }

        if acc % 2 == 1 {
            self.program_string.push_text(LOGICAL_NOT_OPERATOR);
        }

        let part = parts.next().ok_or(IteratorError)?;

        match part.as_rule() {
            Rule::identifier => self.identifier(part),
            Rule::functionCall => self.function_call(part, Some(expression)),
            Rule::property => self.property(part),
            Rule::literal => self.literal(),
            _ => unimplemented!(),
        }
    }

    fn operator(&mut self, operator_rule: Pair<Rule>) -> Result<Type, TranspilerError> {
        // Exract operator type from operator rule
        let operator = operator_rule.into_inner().next().ok_or(IteratorError)?;

        let data = match operator.as_rule() {
            Rule::additionOperator => (ADDITION_OPERATOR, Type::Number),
            Rule::subtractionOperator => (SUBTRACTION_OPERATOR, Type::Number),
            _ => unreachable!(),
        };

        self.program_string.push_text(data.0);

        Ok(data.1)
    }

    fn identifier(&mut self, identifier: Pair<Rule>) -> Result<Type, TranspilerError> {
        // Check if identifier exists in symbol table
        let variable = self.symbol_table.get_variable_or(
            &identifier,
            InvalidVariable {
                identifier: identifier.as_str().to_string(),
            },
        )?;

        self.program_string.push_text(&variable.minified_name);

        Ok(variable.data_type.clone())
    }

    fn property(&mut self, term: Pair<Rule>) -> Result<Type, TranspilerError> {
        // Extract identifier and property
        let mut parts = term.into_inner();
        let identifier = parts.next().ok_or(IteratorError)?;
        let property = parts.next().ok_or(IteratorError)?;

        // Check if identifier exists in symbol table
        let variable = self.symbol_table.get_variable_or(
            &identifier,
            InvalidVariable {
                identifier: identifier.as_str().to_string(),
            },
        )?;

        let error = Err(InvalidProperty {
            identifier: identifier.as_str().to_string(),
            property: property.as_str().to_string(),
        });

        // Short-circuit if variable.data_type is not a string
        if !matches!(variable.data_type, Type::String) {
            return error;
        }

        self.program_string.push_text(&variable.minified_name);
        self.program_string.push_text(DOT);

        // If the function call is in an expression, and the function returns void, throw a void error
        match property.as_str() {
            "length" => {
                self.program_string.push_text(LENGTH_PROPERTY);
                Ok(Type::Number)
            }
            "upper" => {
                self.program_string.push_text(TO_UPPER_CASE_FUNCTION);
                Ok(Type::String)
            }
            "lower" => {
                self.program_string.push_text(TO_LOWER_CASE_FUNCTION);
                Ok(Type::String)
            }
            _ => error,
        }
    }

    fn function_call(
        &mut self,
        function_call: Pair<Rule>,
        expression: Option<&str>,
    ) -> Result<Type, TranspilerError> {
        let mut parts = function_call.into_inner().peekable();

        // if let Rule::identifier = parts.peek().ok_or(IteratorError)?.as_rule() {
        //     let identifier = parts.next().ok_or(IteratorError)?;

        //     let variable = self.symbol_table.get_variable_or(
        //         &identifier,
        //         InvalidFunction {
        //             identifier: identifier.as_str().to_string(),
        //         },
        //     )?;

        //     //todo set up per-variable functions
        // }

        let identifier = parts.next().ok_or(IteratorError)?;

        let function = self.symbol_table.get_function_or(
            &identifier,
            InvalidFunction {
                identifier: identifier.as_str().to_string(),
            },
        )?;

        let returns = function.returns.clone();

        if let Some(expression) = expression {
            if let Type::Void = function.returns {
                return Err(VoidError {
                    identifier: identifier.as_str().to_string(),
                    expression: expression.to_string(),
                });
            }
        }

        self.program_string.push_text(&function.minified_name);
        self.program_string.push_text(OPEN_PARENTHESIS);

        self.function_parameters(parts.next().ok_or(IteratorError)?)?;

        self.program_string.push_text(CLOSE_PARENTHESIS);

        Ok(returns)
    }

    fn function_parameters(&mut self, function_call: Pair<Rule>) -> Result<(), TranspilerError> {
        let parts = function_call.into_inner();

        for expression in parts {
            self.expression(expression)?;
            self.program_string.push_text(COMMA);
        }

        Ok(())
    }

    fn literal(&mut self) -> Result<Type, TranspilerError> {
        self.program_string.push_text("1");

        Ok(Type::String)
    }

    fn end_statement(&mut self) {
        self.program_string.push_text(SEMICOLON);
    }

    pub fn get_program_string(self) -> String {
        self.program_string.text
    }
}
