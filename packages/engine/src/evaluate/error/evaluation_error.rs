use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum NodeEvaluationError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("invalid number of values")]
    InvalidNumberOfValues,

    #[error("unknown symbol")]
    UnknownSymbol,

    #[error("invalid number of arguments")]
    InvalidNumberOfArguments,

    #[error("division by zero is not allowed")]
    DivisionByZero,

    #[error("{0}")]
    ArithmeticError(String),
}
