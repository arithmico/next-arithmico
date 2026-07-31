pub use error_kind::*;
use node::DowncastNodeError;
use thiserror::Error;
use trace::Trace;
use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

pub mod division_by_zero;
mod error_kind;
pub mod incompatible_matrix_dimensions;
pub mod incompatible_tensor_shapes;
pub mod incompatible_vector_dimensions;
pub mod invalid_node;
pub mod invalid_parameter_type;
pub mod invalid_parameter_value;
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
    trace: Trace,
    message: TranslatedMessage,
}

impl EvaluateNodeError {
    fn new(kind: EvaluateNodeErrorKind, message: TranslatedMessage) -> Self {
        Self {
            kind,
            trace: Trace::new(),
            message,
        }
    }

    pub fn get_message(&self) -> &TranslatedMessage {
        &self.message
    }

    pub fn get_error_kind(&self) -> EvaluateNodeErrorKind {
        self.kind.clone()
    }

    pub fn stack_trace(&self) -> &Trace {
        &self.trace
    }
}

impl AsRef<Trace> for EvaluateNodeError {
    fn as_ref(&self) -> &Trace {
        &self.trace
    }
}

impl AsMut<Trace> for EvaluateNodeError {
    fn as_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl From<DowncastNodeError> for EvaluateNodeError {
    fn from(value: DowncastNodeError) -> Self {
        let mut err = Self::new(
            EvaluateNodeErrorKind::UnexpectedNodeType,
            TranslatedMessage::new(
                "engine.evaluate.error.unexpected_node_type",
                translation_resolver,
            )
            .key("expected", value.expected.to_string())
            .key("received", value.received.to_string()),
        );
        err.trace = value.trace;
        err
    }
}
