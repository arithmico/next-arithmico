use std::collections::HashSet;

use ast::NodeType;
use thiserror::Error;
use trace::{IntoTrace, Trace};

// TODO: extract inner error into separate file
// TODO: replace format strings with dummy names
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

    #[error("Missing parameter \"{0}\"")]
    MissingParameter(String),

    #[error("Invalid repeatable parameter count. Expected between {min} and {max:?} but received {received}.")]
    InvalidRepeatableParameterCount {
        name: String,
        min: usize,
        max: Option<usize>,
        received: usize,
    },

    #[error("InvalidParameterType")]
    InvalidParameterType {
        name: String,
        expected: HashSet<NodeType>,
        received: NodeType,
    },

    #[error("TooManyParameters")]
    TooManyParameters,
}

// TODO: add translations later
#[derive(Error, Debug, PartialEq, Clone)]
#[error("{inner}")]
pub struct EvaluateNodeError {
    inner: EvaluateNodeInnerError,
    stack_trace: Vec<Trace>,
}

// TODO: extract variant constructor methods into seperate files
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

    pub fn unsupported_datatype(node_kind: NodeType) -> Self {
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

    pub fn missing_parameter(name: impl ToString) -> Self {
        Self::from_inner(EvaluateNodeInnerError::MissingParameter(
            name.to_string(),
        ))
    }

    pub fn invalid_repeatable_parameter_count(
        name: String,
        min: usize,
        max: Option<usize>,
        received: usize,
    ) -> Self {
        Self::from_inner(
            EvaluateNodeInnerError::InvalidRepeatableParameterCount {
                name,
                min,
                max,
                received,
            },
        )
    }

    pub fn invalid_parameter_type(
        name: String,
        expected: HashSet<NodeType>,
        received: NodeType,
    ) -> Self {
        Self::from_inner(EvaluateNodeInnerError::InvalidParameterType {
            name,
            expected,
            received,
        })
    }

    pub fn too_many_parameters(_remaining_parameter_count: usize) -> Self {
        Self::from_inner(EvaluateNodeInnerError::TooManyParameters)
    }

    pub fn with_tracable<T: IntoTrace>(mut self, tracable: T) -> Self {
        let trace = tracable.into_trace();
        if !trace.is_empty() {
            self.stack_trace.push(trace);
        }
        self
    }

    pub fn stack_trace(&self) -> Vec<Trace> {
        self.stack_trace.clone()
    }
}
