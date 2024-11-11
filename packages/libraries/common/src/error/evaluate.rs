use std::collections::{HashMap, HashSet};

use ast::NodeType;
pub use error_kind::*;
use thiserror::Error;
use trace::{IntoTrace, Trace};
use translate_core::{Language, Translatable, TranslatedMessage};

mod error_kind;

#[derive(Error, Debug, PartialEq, Clone)]
#[error("{inner:?}")]
pub struct EvaluateNodeError {
    inner: EvaluateNodeErrorKind,
    stack_trace: Vec<Trace>,
    message: TranslatedMessage,
    keys: HashMap<String, String>,
}

// TODO: extract variant constructor methods into seperate files
impl EvaluateNodeError {
    fn from_error_kind(inner: EvaluateNodeErrorKind) -> Self {
        Self {
            inner,
            stack_trace: Vec::new(),
            message: TranslatedMessage::new(),
            keys: HashMap::new(),
        }
    }

    pub fn unsupported_operation() -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::UnsupportedOperation)
    }

    pub fn unsupported_datatype(node_kind: NodeType) -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::UnsupportedDataType)
    }

    pub fn unknown_symbol<T: ToString>(name: T) -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::UnknownSymbol)
    }

    pub fn runtime_error<T: ToString>(message: T) -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::RuntimeError)
    }

    pub fn invalid_node<T: ToString>(node_kind: T) -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::InvalidNode)
    }

    pub fn incompatible_vector_dimensions(left: usize, right: usize) -> Self {
        Self::from_error_kind(
            EvaluateNodeErrorKind::IncompatibleVectorDimensions,
        )
    }

    pub fn incompatible_matrix_dimensions(
        left: Vec<usize>,
        right: Vec<usize>,
    ) -> Self {
        Self::from_error_kind(
            EvaluateNodeErrorKind::IncompatibleMatrixDimensions,
        )
    }

    pub fn division_by_zero() -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::DivisionByZero)
    }

    pub fn missing_parameter(name: impl ToString) -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::MissingParameter)
    }

    pub fn invalid_repeatable_parameter_count(
        name: String,
        min: usize,
        max: Option<usize>,
        received: usize,
    ) -> Self {
        Self::from_error_kind(
            EvaluateNodeErrorKind::InvalidRepeatableParameterCount,
        )
    }

    pub fn invalid_parameter_type(
        name: String,
        expected: HashSet<NodeType>,
        received: NodeType,
    ) -> Self {
        Self::from_error_kind(EvaluateNodeErrorKind::InvalidParameterType)
    }

    pub fn too_many_parameters(remaining_parameter_count: usize) -> Self {
        let mut error =
            Self::from_error_kind(EvaluateNodeErrorKind::TooManyParameters);
        error.message.add_translation(Language::English, "Too many parameters: the last {count} parameters could not be mapped to function inputs.");
        error.message.add_translation(Language::German, "Zu viele Parameter: Die letzten {count} Parameter konnten nicht auf Funktions-Eingaben abgebildet werden.");
        error
            .keys
            .insert("count".to_string(), remaining_parameter_count.to_string());
        error
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

impl Translatable for EvaluateNodeError {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, translate_core::TranslationError> {
        self.message.translate_with(language, &self.keys)
    }
}
