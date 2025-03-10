pub use ast::EvaluateNodeError;
use ast::ParseNodeError;
pub use ast::SerializeNodeError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("ParserError: {0:?}")]
    ParseNodeError(#[from] ParseNodeError),

    #[error("SerializationError: {0:?}")]
    SerializeNodeError(#[from] SerializeNodeError),

    #[error("RuntimeError: {0:?}")]
    EvaluateNodeError(#[from] EvaluateNodeError),
}
