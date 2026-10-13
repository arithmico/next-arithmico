use serde::{Deserialize, Serialize};
use thiserror::Error;
use trace::{Span, Tracable};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SessionError {
    #[error("ParseError: {0:?}")]
    ParseNodeError(#[from] parser::Error),

    #[error("EvaluateError: {0:?}")]
    EvaluateNodeError(#[from] evaluator::Error),
}

impl SessionError {
    pub fn get_spans(&self) -> Vec<Span> {
        match self {
            SessionError::ParseNodeError(error) => vec![error.get_span()],
            SessionError::EvaluateNodeError(error) => {
                error.trace().first_spans().collect()
            }
        }
    }
}
