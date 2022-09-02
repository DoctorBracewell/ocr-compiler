use std::collections::HashMap;

use pest::iterators::Pair;

use super::ast_walker::Type;
use crate::parser::Rule;
use crate::utils::denary_to_alphabet;

// Symbol table entry, one per identifier
#[derive(Debug, Clone)]
pub struct VariableTableEntry {
    pub minified_name: String,
    pub data_type: Type,
    constant: bool,
}

pub struct FunctionTableEntry {
    pub minified_name: String,
    pub returns: Type,
}

#[derive(Default)]
pub struct SymbolTable {
    pub variables: HashMap<String, VariableTableEntry>,
    pub functions: HashMap<String, FunctionTableEntry>,
    pub attatched_functions: HashMap<String, FunctionTableEntry>,
}

impl SymbolTable {
    pub fn insert_or_update_variable(
        &mut self,
        identifier: &Pair<Rule>,
        data_type: Type,
    ) -> &mut VariableTableEntry {
        let table_size = self.total_size();
        let identifier_string = identifier.as_str().to_string();

        self.variables
            .entry(identifier_string)
            .insert_entry(VariableTableEntry {
                minified_name: denary_to_alphabet(table_size),
                // functions: self.determine_functions(&data_type),
                data_type,
                constant: false,
            })
            .into_mut()
    }

    pub fn get_variable_or<E>(
        &mut self,
        identifier: &Pair<Rule>,
        error: E,
    ) -> Result<&VariableTableEntry, E> {
        let identifier_string = identifier.as_str().to_string();

        self.variables.get(&identifier_string).ok_or(error)
    }

    pub fn set_variable_type_or<E>(
        &mut self,
        identifier: &Pair<Rule>,
        data_type: Type,
        error: E,
    ) -> Result<(), E> {
        let identifier_string = identifier.as_str().to_string();

        let mut entry = self.variables.get_mut(&identifier_string).ok_or(error)?;

        entry.data_type = data_type;

        Ok(())
    }

    pub fn insert_or_update_function(
        &mut self,
        identifier: &Pair<Rule>,
        returns: Type,
    ) -> &mut FunctionTableEntry {
        let table_size = self.total_size();
        let identifier_string = identifier.as_str().to_string();

        self.functions
            .entry(identifier_string)
            .insert_entry(FunctionTableEntry {
                minified_name: denary_to_alphabet(table_size),
                returns,
            })
            .into_mut()
    }

    pub fn get_function_or<E>(
        &self,
        identifier: &Pair<Rule>,
        error: E,
    ) -> Result<&FunctionTableEntry, E> {
        let identifier_string = identifier.as_str().to_string();

        self.functions.get(&identifier_string).ok_or(error)
    }

    // fn determine_functions(data_type: Type) -> Vec<FunctionTableEntry> {
    //     match data_type {
    //         Type::Number => {
    //             return vec![Type::Number, Type::String];
    //         }
    //         Type::String => {
    //             return vec![Type::String, Type::Number];
    //         }
    //         _ => {
    //             return vec![data_type];
    //         }
    //     }
    // }

    fn total_size(&self) -> usize {
        self.variables.len() + self.functions.len()
    }
}

mod inbuilt_functions {
    use crate::transpiler::ast_walker::Type;

    pub struct FunctionData {
        pub returns: Type,
        // pub arguments: Vec<Type>,
    }

    fn substring() -> FunctionData {
        FunctionData {
            returns: Type::String,
        }
    }
}
