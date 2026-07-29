use thiserror::Error;

use crate::core::{EvaluateNodeError, SerializeNodeError};

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("ParseError: {0:?}")]
    ParseNodeError(#[from] parser::Error),

    #[error("SerializationError: {0:?}")]
    SerializeNodeError(#[from] SerializeNodeError),

    #[error("RuntimeError: {0:?}")]
    EvaluateNodeError(#[from] EvaluateNodeError),
}
