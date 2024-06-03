use pest::error::Error;
use thiserror::Error;

use crate::core::node::*;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluationError {
    #[error("SyntaxError: {0}")]
    SyntaxError(Error<Rule>),

    #[error("RuntimeError: {0}")]
    RuntimeError(NodeError),
}

impl From<Error<Rule>> for EvaluationError {
    fn from(value: Error<Rule>) -> Self {
        EvaluationError::SyntaxError(value)
    }
}

impl From<NodeError> for EvaluationError {
    fn from(value: NodeError) -> Self {
        EvaluationError::RuntimeError(value)
    }
}
