use thiserror::Error;

use crate::core::EvaluateNodeError;

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("ParseError: {0:?}")]
    ParseNodeError(#[from] parser::Error),

    #[error("SerializationError: {0:?}")]
    SerializeNodeError(#[from] serializer::Error),

    #[error("RuntimeError: {0:?}")]
    EvaluateNodeError(#[from] EvaluateNodeError),
}
