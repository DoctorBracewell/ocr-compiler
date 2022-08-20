use std::collections::{btree_map::OccupiedEntry, HashMap};

use pest::iterators::Pair;

use super::errors::{TranspilerError::*, *};
use crate::parser::Rule;
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
    pub fn insert_or_update_variable(&mut self, identifier: &Pair<Rule>) -> &mut SymbolTableEntry {
        let table_size = self.variables.len();
        let identifier_string = identifier.as_str().to_string();

        self.variables
            .entry(identifier_string)
            .insert_entry(SymbolTableEntry {
                minified_name: denary_to_alphabet(table_size),
                constant: false,
            })
            .into_mut()
    }

    pub fn get_variable_or<E>(
        &mut self,
        identifier: &Pair<Rule>,
        error: E,
    ) -> Result<&SymbolTableEntry, E> {
        let identifier_string = identifier.as_str().to_string();

        self.variables.get(&identifier_string).ok_or(error)
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
