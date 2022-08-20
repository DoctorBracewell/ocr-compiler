use std::collections::HashMap;

use super::errors::TranspilerError;
use crate::utils::denary_to_alphabet;

type Table = HashMap<String, SymbolTableEntry>;

// Symbol table entry, one per identifier
#[derive(Debug, Clone)]
pub struct SymbolTableEntry {
    pub minified_name: String,
    constant: bool,
}

#[derive(Default)]
pub struct SymbolTable {
    pub variables: Table,
    pub functions: Table,
}

impl SymbolTable {
    pub fn add_variable(&mut self, identifier: &str) -> Result<(), TranspilerError> {
        let table = &mut self.variables;
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

    pub fn add_function(&mut self, identifier: &str) -> Result<(), TranspilerError> {
        let table = &mut self.functions;
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
}
