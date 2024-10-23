use thiserror::Error;
use trace::Trace;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvaluateNodeInnerError {
    #[error("unsupported operation")]
    UnsupportedOperation,

    #[error("unsupported data type '{0}'")]
    UnsupportedDataType(String),

    #[error("unknown symbol '{0}'")]
    UnknownSymbol(String),

    #[error("RuntimeError: {0}")]
    RuntimeError(String),

    #[error("invalid node")]
    InvalidNode(String),

    #[error("incompatible vector dimensions: {0}, {1}")]
    IncompatibleVectorDimensions(usize, usize),

    #[error("incompatible vector dimensions: {0:?}, {1:?}")]
    IncompatibleMatrixDimensions(Vec<usize>, Vec<usize>),

    #[error("division by zero")]
    DivisionByZero,

    #[error("Invalid number of arguments: Expected {0} got {1}")]
    InvalidNumberOfArguments(usize, usize),
}

#[derive(Error, Debug, PartialEq, Clone)]
#[error("{inner}")]
pub struct EvaluateNodeError {
    inner: EvaluateNodeInnerError,
    stack_trace: Vec<Trace>,
}

impl EvaluateNodeError {
    fn from_inner(inner: EvaluateNodeInnerError) -> Self {
        Self {
            inner,
            stack_trace: Vec::new(),
        }
    }

    pub fn unsupported_operation() -> Self {
        Self::from_inner(EvaluateNodeInnerError::UnsupportedOperation)
    }

    pub fn unsupported_datatype<T: ToString>(node_kind: T) -> Self {
        Self::from_inner(EvaluateNodeInnerError::UnsupportedDataType(
            node_kind.to_string(),
        ))
    }

    pub fn unknown_symbol<T: ToString>(name: T) -> Self {
        Self::from_inner(EvaluateNodeInnerError::UnknownSymbol(
            name.to_string(),
        ))
    }

    pub fn runtime_error<T: ToString>(message: T) -> Self {
        Self::from_inner(EvaluateNodeInnerError::RuntimeError(
            message.to_string(),
        ))
    }

    pub fn invalid_node<T: ToString>(node_kind: T) -> Self {
        Self::from_inner(EvaluateNodeInnerError::InvalidNode(
            node_kind.to_string(),
        ))
    }

    pub fn incompatible_vector_dimensions(left: usize, right: usize) -> Self {
        Self::from_inner(EvaluateNodeInnerError::IncompatibleVectorDimensions(
            left, right,
        ))
    }

    pub fn incompatible_matrix_dimensions(
        left: Vec<usize>,
        right: Vec<usize>,
    ) -> Self {
        Self::from_inner(EvaluateNodeInnerError::IncompatibleMatrixDimensions(
            left, right,
        ))
    }

    pub fn division_by_zero() -> Self {
        Self::from_inner(EvaluateNodeInnerError::DivisionByZero)
    }

    pub fn invalid_number_of_arguments(
        expected: usize,
        received: usize,
    ) -> Self {
        Self::from_inner(EvaluateNodeInnerError::InvalidNumberOfArguments(
            expected, received,
        ))
    }
}
