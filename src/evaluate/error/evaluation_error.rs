use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NodeEvaluationError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("invalid number of values")]
    InvalidNumberOfValues,

    #[error("unknown symbol")]
    UnknownSymbol,

    #[error("invalid number of arguments")]
    InvalidNumberOfArguments,
}
