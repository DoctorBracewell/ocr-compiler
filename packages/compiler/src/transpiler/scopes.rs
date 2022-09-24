use std::collections::HashMap;

use pest::iterators::Pair;

use super::ast_walker::Type;
use crate::parser::Rule;
use crate::utils::denary_to_alphabet;

pub struct Scope {
    scopes: HashMap<String, Scope>,
    identifier: String,
    minified_name: String,
    variables: VariableTable,
    parent: Option<Box<Scope>>,
    returns: Type,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            identifier: "global".to_string(),
            minified_name: "global".to_string(),
            ..Default::default()
        }
    }
}

impl Scope {
    fn test(&mut self) {}
}

type VariableTable = HashMap<String, VariableTableEntry>;

impl VariableTable {
    fn insert_or_update(
        &mut self,
        identifier: &Pair<Rule>,
        variable_type: Type,
        size: usize,
    ) -> &mut VariableTableEntry {
        let identifier_string = identifier.as_str().to_string();

        self.entry(identifier_string)
            .insert_entry(VariableTableEntry {
                minified_name: denary_to_alphabet(size),
                // functions: self.determine_functions(&data_type),
                variable_type,
            })
            .into_mut()
    }

    fn get_or<E>(&mut self, identifier: &Pair<Rule>, error: E) -> Result<&VariableTableEntry, E> {
        let identifier_string = identifier.as_str().to_string();

        self.get(&identifier_string).ok_or(error)
    }
}

#[derive(Clone)]
pub struct VariableTableEntry {
    pub minified_name: String,
    pub variable_type: Type,
}

impl SymbolTable<VariableTableEntry> for VariableTable {}

// Function Table
struct FunctionTable {
    table: HashMap<String, FunctionTableEntry>,
}

pub struct FunctionTableEntry {
    pub minified_name: String,
    pub return_type: Type,
}

impl SymbolTable<FunctionTableEntry> for FunctionTable {
    fn insert_or_update(
        &mut self,
        identifier: &Pair<Rule>,
        return_type: Type,
        size: usize,
    ) -> &mut FunctionTableEntry {
        let identifier_string = identifier.as_str().to_string();

        self.table
            .entry(identifier_string)
            .insert_entry(FunctionTableEntry {
                minified_name: denary_to_alphabet(size),
                // functions: self.determine_functions(&data_type),
                return_type,
            })
            .into_mut()
    }
}

impl SymbolTables {
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
