use trace::{IntoTrace, Span};

use super::EvaluateNodeError;

#[derive(Debug, Clone, PartialEq)]
pub struct UnsupportedOperationError {
    spans: Vec<Span>,
}

impl UnsupportedOperationError {
    pub fn new(spans: Vec<Span>) -> Self {
        Self { spans }
    }
}

impl From<UnsupportedOperationError> for EvaluateNodeError {
    fn from(value: UnsupportedOperationError) -> Self {
        Self::UnsupportedOperation(value)
    }
}

impl<T: IntoTrace> From<T> for UnsupportedOperationError {
    fn from(value: T) -> Self {
        Self::new(value.into_trace().spans())
    }
}
