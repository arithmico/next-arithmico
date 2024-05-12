use crate::parse::parser::Rule;
use pest::error::Error;
use thiserror::Error;

use crate::evaluate::NodeEvaluationError;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluationError {
    #[error("SyntaxError: {0}")]
    SyntaxError(Error<Rule>),

    #[error("RuntimeError: {0}")]
    RuntimeError(NodeEvaluationError),
}

impl From<Error<Rule>> for EvaluationError {
    fn from(value: Error<Rule>) -> Self {
        EvaluationError::SyntaxError(value)
    }
}

impl From<NodeEvaluationError> for EvaluationError {
    fn from(value: NodeEvaluationError) -> Self {
        EvaluationError::RuntimeError(value)
    }
}
