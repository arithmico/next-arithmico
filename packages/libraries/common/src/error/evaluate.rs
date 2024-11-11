use std::collections::HashSet;

use ast::NodeType;
pub use error_kind::*;
use thiserror::Error;
use trace::{IntoTrace, Trace};
use translate_core::{Language, Translatable, TranslatedMessage};

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
            TranslatedMessage::new()
                .translation(Language::English, "Unsupported operation")
                .translation(Language::German, "Nicht unterstützte Operation"),
        )
    }

    pub fn unsupported_datatype(node_kind: NodeType) -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnsupportedDataType,
            TranslatedMessage::new()
                .translation(
                    Language::English,
                    "Unsupported datatype: \"{node_kind}\"",
                )
                .translation(
                    Language::German,
                    "Nicht unterstützter Datentyp: \"{node_kind}\"",
                )
                .key("node_kind", node_kind),
        )
    }

    pub fn unknown_symbol<T: ToString>(name: T) -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnknownSymbol,
            TranslatedMessage::new()
                .translation(Language::English, "Unknown symbol: \"{name}\"")
                .translation(
                    Language::English,
                    "Unbekanntes Symbol: \"{name}\"",
                )
                .key("name", name),
        )
    }

    // TODO: make T: Translatable
    pub fn runtime_error<T: ToString>(message: T) -> Self {
        Self::new(
            EvaluateNodeErrorKind::RuntimeError,
            TranslatedMessage::new()
                .translation(Language::English, "Runtime error: {message}")
                .translation(Language::English, "Laufzeitfehler: {message}")
                .key("message", message),
        )
    }

    pub fn invalid_node(node_type: NodeType) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidNode,
            TranslatedMessage::new()
                .translation(
                    Language::English,
                    "Invalid node of type \"{node_type}\"",
                )
                .translation(
                    Language::English,
                    "Ungültiger Knoten vom Typ \"{node_type}\"",
                )
                .key("node_type", node_type),
        )
    }

    pub fn incompatible_vector_dimensions(left: usize, right: usize) -> Self {
        Self::new(
            EvaluateNodeErrorKind::IncompatibleVectorDimensions,
            TranslatedMessage::new()
                .translation(
                    Language::English,
                    "Incompatible vector dimensions: {left} and {right}",
                )
                .translation(
                    Language::German,
                    "Inkompatible Vektordimensionen: {left} und {right}",
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
            TranslatedMessage::new()
                .translation(
                    Language::English,
                    "Incompatible matrix dimensions: {left} and {right}",
                )
                .translation(
                    Language::English,
                    "Inkompatible Matrixdimensionen: {left} und {right}",
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
            TranslatedMessage::new()
                .translation(Language::English, "Division by zero")
                .translation(Language::German, "Division durch Null"),
        )
    }

    pub fn missing_parameter(name: impl ToString) -> Self {
        Self::new(
            EvaluateNodeErrorKind::MissingParameter,
            TranslatedMessage::new()
                .translation(
                    Language::English,
                    "Missing parameter value for \"{name}\"",
                )
                .translation(
                    Language::German,
                    "Fehlender Parameterwert für \"{name}\"",
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
        Self::new(EvaluateNodeErrorKind::InvalidRepeatableParameterCount, TranslatedMessage::new()
        .translation(Language::English, "Invalid number of values for the repeatable parameter \"{name}\". Expected between {min} and {max} values but received {received}.")
        .translation(Language::German, "Ungültige Anzahl von Parameterwerten für den wiederholbaren Parameter \"{name}\". Es werden zwischen {min} und {max} Werte. Es wurden {received} Werte übergeben.")
        .key("name", name)
        .key("min", min)
        .key("max", format!("{:?}", max))
        .key("received", received)
    )
    }

    pub fn invalid_parameter_type(
        name: String,
        expected: HashSet<NodeType>,
        received: NodeType,
    ) -> Self {
        Self::new(EvaluateNodeErrorKind::InvalidParameterType, TranslatedMessage::new()
            .translation(Language::English, "Invalid parameter type for \"{name}\". Expected {expected} received {received}.")
            .translation(Language::German, "Ungültiger Parameterwert für \"{name}\". Es wurde eine Wert vom Typ {expected} erwartet. Stattdessen wurde ein Wert vom Typ {received} gefunden.")
            .key("name", name)
            .key("expected", expected.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
            .key("received", received))
    }

    pub fn too_many_parameters(count: usize) -> Self {
        Self::new(EvaluateNodeErrorKind::TooManyParameters, TranslatedMessage::new()
            .translation(Language::English, "Too many parameters: the last {count} parameters could not be mapped to function inputs.")
            .translation(Language::German, "Zu viele Parameter: Die letzten {count} Parameter konnten nicht auf Funktions-Eingaben abgebildet werden.")
            .key("count", count))
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
