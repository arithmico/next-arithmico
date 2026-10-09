use thiserror::Error;
use trace::{Span, Tracable};

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("ParseError: {0:?}")]
    ParseNodeError(#[from] parser::Error),

    #[error("SerializeError: {0:?}")]
    SerializeNodeError(#[from] serializer::Error),

    #[error("EvaluateError: {0:?}")]
    EvaluateNodeError(#[from] evaluator::Error),
}

impl SessionError {
    pub fn get_spans(&self) -> Vec<Span> {
        match self {
            SessionError::ParseNodeError(error) => vec![error.get_span()],
            SessionError::SerializeNodeError(error) => {
                error.get_spans().collect()
            }
            SessionError::EvaluateNodeError(error) => {
                error.trace().first_spans().collect()
            }
        }
    }
}
