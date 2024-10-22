use thiserror::Error;

mod unsuppored_operation;

pub use unsuppored_operation::*;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluateNodeError {
    #[error("unsupported operation")]
    UnsupportedOperation(UnsupportedOperationError),

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

    #[error("division by zero")]
    DivisionByZero,

    #[error("Invalid number of arguments: Expected {0} got {1}")]
    InvalidNumberOfArguments(usize, usize),
}

impl EvaluateNodeError {
    pub fn unsupported_operation(
        from: impl Into<UnsupportedOperationError>,
    ) -> Self {
        Self::UnsupportedOperation(from.into())
    }
}
