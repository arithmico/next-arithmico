use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluateNodeError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("unsupported data type '{0}'")]
    UnsupportedDataType(String),

    #[error("RuntimeError: {0}")]
    RuntimeError(String),
}
