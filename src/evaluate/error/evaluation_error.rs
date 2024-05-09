use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("invalid number of values")]
    InvalidNumberOfValues,

    #[error("unknown symbol")]
    UnknownSymbol,

    #[error("invalid number of arguments")]
    InvalidNumberOfArguments,
}
