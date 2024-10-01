use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluateNodeError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("RuntimeError: {0}")]
    RuntimeError(String),
}
