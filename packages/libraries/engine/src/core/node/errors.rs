use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum NodeError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("RuntimeError: {0}")]
    RuntimeError(String),

    #[error("ParsingError: {0}")]
    ParsingError(String),
}
