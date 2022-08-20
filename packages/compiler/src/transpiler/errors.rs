use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranspilerError {
    #[error("Constant variable {identifier} reassignment attempted at")]
    ConstantAssignment { identifier: String },
    #[error("Constant variable ")]
    InvalidReference,
    #[error("Iterator Error at . This likely occured because invalid syntax was not caught by the parser.")]
    IteratorError,
    #[error("Some internal data structure used by the transpiler failed at")]
    TranspilationError,
}

/*
- Redeclare variable
- Refernce bet
*/
