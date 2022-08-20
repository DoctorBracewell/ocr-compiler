use std::collections::HashMap;

use super::errors::{TranspilerError::*, *};
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
    pub fn get_or_add_variable(&mut self, identifier: String) -> &mut SymbolTableEntry {
        let table_size = self.variables.len();

        self.variables
            .entry(identifier)
            .or_insert_with(|| SymbolTableEntry {
                minified_name: denary_to_alphabet(table_size),
                constant: false,
            })
    }

    pub fn get_or_add_function(&mut self, identifier: String) -> &mut SymbolTableEntry {
        let table_size = self.functions.len();

        self.functions
            .entry(identifier)
            .or_insert_with(|| SymbolTableEntry {
                minified_name: denary_to_alphabet(table_size),
                constant: false,
            })
    }
}
