use thiserror::Error;

use crate::core::{EvaluateNodeError, ParseNodeError, SerializeNodeError};

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("ParserError: {0:?}")]
    ParseNodeError(#[from] ParseNodeError),

    #[error("SerializationError: {0:?}")]
    SerializeNodeError(#[from] SerializeNodeError),

    #[error("RuntimeError: {0:?}")]
    EvaluateNodeError(#[from] EvaluateNodeError),
}
