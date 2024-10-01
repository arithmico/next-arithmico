use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluationError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("RuntimeError: {0}")]
    RuntimeError(String),
}
