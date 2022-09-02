use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranspilerError {
    #[error("Constant variable {identifier} reassignment attempted at")]
    ConstantAssignment { identifier: String },
    #[error("Variable {identifier} used illegally at. This might mean that you have not initialised the variable, or you are using it before you have initialised it.")]
    InvalidVariable { identifier: String },
    #[error("Function {identifier}(...) used illegally at. This might mean that you have not defined the function, or you are calling it before you have defined it.")]
    InvalidFunction { identifier: String },
    #[error("Function {identifier} returns void, this value cannot be used in the expression {expression}")]
    VoidError {
        identifier: String,
        expression: String,
    },
    #[error("Property {property} does not exist on {identifier}")]
    InvalidProperty {
        identifier: String,
        property: String,
    },
    #[error("Iterator Error at . This likely occured because invalid syntax was not caught by the parser.")]
    IteratorError,
    #[error("Some internal data structure used by the transpiler failed at")]
    TranspilationError,
}

/*
- Redeclare variable
- Refernce bet
*/
