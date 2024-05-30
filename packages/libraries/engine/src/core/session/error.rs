use pest::error::Error;
use thiserror::Error;

use crate::core::{node::*, parse::Rule};

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluationError {
    #[error("SyntaxError: {0}")]
    SyntaxError(Error<Rule>),

    #[error("RuntimeError: {0}")]
    RuntimeError(EvaluateNodeError),
}

impl From<Error<Rule>> for EvaluationError {
    fn from(value: Error<Rule>) -> Self {
        EvaluationError::SyntaxError(value)
    }
}

impl From<EvaluateNodeError> for EvaluationError {
    fn from(value: EvaluateNodeError) -> Self {
        EvaluationError::RuntimeError(value)
    }
}
