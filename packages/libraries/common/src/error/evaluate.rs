use std::collections::HashSet;

use ast::NodeType;
pub use error_kind::*;
use thiserror::Error;
use trace::{IntoTrace, Trace};
use translate_core::{Language, Translatable, TranslatedMessage};

use crate::translation_provider::translation_resolver;

mod error_kind;

#[derive(Error, Debug, PartialEq, Clone)]
#[error("{kind:?}")]
pub struct EvaluateNodeError {
    kind: EvaluateNodeErrorKind,
    stack_trace: Vec<Trace>,
    message: TranslatedMessage,
}

// TODO: extract variant constructor methods into seperate files
impl EvaluateNodeError {
    fn new(kind: EvaluateNodeErrorKind, message: TranslatedMessage) -> Self {
        Self {
            kind,
            stack_trace: Vec::new(),
            message,
        }
    }

    pub fn unsupported_operation() -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnsupportedOperation,
            TranslatedMessage::new(
                "engine.evaluate.error.unsupported_operation",
                translation_resolver,
            ),
        )
    }

    pub fn unsupported_datatype(node_kind: NodeType) -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnsupportedDataType,
            TranslatedMessage::new(
                "engine.evaluate.error.unsupported_datatype",
                translation_resolver,
            )
            .key("node_kind", node_kind),
        )
    }

    pub fn unknown_symbol<T: ToString>(name: T) -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnknownSymbol,
            TranslatedMessage::new(
                "engine.evaluate.error.unknown_symbol",
                translation_resolver,
            )
            .key("name", name),
        )
    }

    // TODO: make T: Translatable
    pub fn runtime_error<T: ToString>(message: T) -> Self {
        Self::new(
            EvaluateNodeErrorKind::RuntimeError,
            TranslatedMessage::new(
                "engine.evaluate.error.runtime_error",
                translation_resolver,
            )
            .key("message", message),
        )
    }

    pub fn invalid_node(node_type: NodeType) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidNode,
            TranslatedMessage::new(
                "engine.evaluate.error.invalid_node",
                translation_resolver,
            )
            .key("node_type", node_type),
        )
    }

    pub fn incompatible_vector_dimensions(left: usize, right: usize) -> Self {
        Self::new(
            EvaluateNodeErrorKind::IncompatibleVectorDimensions,
            TranslatedMessage::new(
                "engine.evaluate.error.incompatible_vector_dimensions",
                translation_resolver,
            )
            .key("left", left)
            .key("right", right),
        )
    }

    pub fn incompatible_matrix_dimensions(
        left: Vec<usize>,
        right: Vec<usize>,
    ) -> Self {
        Self::new(
            EvaluateNodeErrorKind::IncompatibleMatrixDimensions,
            TranslatedMessage::new(
                "engine.evaluate.error.incompatible_matrix_dimensions",
                translation_resolver,
            )
            .key(
                "left",
                left.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("x"),
            )
            .key(
                "right",
                right
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("x"),
            ),
        )
    }

    pub fn division_by_zero() -> Self {
        Self::new(
            EvaluateNodeErrorKind::DivisionByZero,
            TranslatedMessage::new(
                "engine.evaluate.error.division_by_zero",
                translation_resolver,
            ),
        )
    }

    pub fn missing_parameter(name: impl ToString) -> Self {
        Self::new(
            EvaluateNodeErrorKind::MissingParameter,
            TranslatedMessage::new(
                "engine.evaluate.error.missing_parameter",
                translation_resolver,
            )
            .key("name", name),
        )
    }

    pub fn invalid_repeatable_parameter_count(
        name: String,
        min: usize,
        max: Option<usize>,
        received: usize,
    ) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidRepeatableParameterCount,
            TranslatedMessage::new(
                "engine.evaluate.error.invalid_repeatable_parameter_count",
                translation_resolver,
            )
            .key("name", name)
            .key("min", min)
            .key("max", format!("{:?}", max))
            .key("received", received),
        )
    }

    pub fn invalid_parameter_type(
        name: String,
        expected: HashSet<NodeType>,
        received: NodeType,
    ) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidParameterType,
            TranslatedMessage::new(
                "engine.evaluate.error.invalid_parameter_type",
                translation_resolver,
            )
            .key("name", name)
            .key(
                "expected",
                expected
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .key("received", received),
        )
    }

    pub fn too_many_parameters(count: usize) -> Self {
        Self::new(
            EvaluateNodeErrorKind::TooManyParameters,
            TranslatedMessage::new(
                "engine.evaluate.error.too_many_parameters",
                translation_resolver,
            )
            .key("count", count),
        )
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
        self.message.translate(language)
    }
}
