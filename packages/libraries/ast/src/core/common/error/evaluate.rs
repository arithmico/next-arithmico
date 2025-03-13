pub use error_kind::*;
use thiserror::Error;
use trace::{IntoTrace, Trace};
use translate_core::TranslatedMessage;

pub mod division_by_zero;
mod error_kind;
pub mod incompatible_matrix_dimensions;
pub mod incompatible_vector_dimensions;
pub mod invalid_node;
pub mod invalid_parameter_type;
pub mod invalid_repeatable_parameter_count;
pub mod missing_parameter;
pub mod runtime_error;
pub mod too_many_parameters;
pub mod unknown_symbol;
pub mod unsupported_datatype;
pub mod unsupported_operation;

#[derive(Error, Debug, PartialEq, Clone)]
#[error("{kind:?}")]
pub struct EvaluateNodeError {
    kind: EvaluateNodeErrorKind,
    stack_trace: Vec<Trace>,
    message: TranslatedMessage,
}

impl EvaluateNodeError {
    fn new(kind: EvaluateNodeErrorKind, message: TranslatedMessage) -> Self {
        Self {
            kind,
            stack_trace: Vec::new(),
            message,
        }
    }

    pub fn get_message(&self) -> &TranslatedMessage {
        &self.message
    }

    pub fn get_error_kind(&self) -> EvaluateNodeErrorKind {
        self.kind.clone()
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
