use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluateNodeError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("unsupported data type '{0}'")]
    UnsupportedDataType(String),

    #[error("unknown symbol '{0}'")]
    UnknownSymbol(String),

    #[error("RuntimeError: {0}")]
    RuntimeError(String),

    #[error("invalid node")]
    InvalidNode,

    #[error("incompatible vector dimensions: {0}, {1}")]
    IncompatibleVectorDimensions(usize, usize),

    #[error("incompatible vector dimensions: {0:?}, {1:?}")]
    IncompatibleMatrixDimensions(Vec<usize>, Vec<usize>),
}
