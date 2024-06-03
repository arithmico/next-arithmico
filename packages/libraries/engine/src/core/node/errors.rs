use pest::error::Error as PestError;
use thiserror::Error;

use super::node::Rule;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum NodeError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("RuntimeError: {0}")]
    RuntimeError(String),

    #[error("ParsingError: {0}")]
    ParsingError(String),

    #[error("SyntaxError: {0}")]
    SyntaxError(#[from] PestError<Rule>),
}
