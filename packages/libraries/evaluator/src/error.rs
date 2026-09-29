use std::collections::HashSet;

use node::NodeType;
use thiserror::Error;
use trace::Trace;
use translate_core::{
    IntoValue, RenderedTranslatedMessage, Translatable, TranslatedMessage,
};

use crate::{ErrorKind, translation_provider::translation_resolver};

#[derive(Error, Debug, PartialEq, Clone)]
#[error("{kind:?}")]
pub struct Error {
    kind: ErrorKind,
    trace: Trace,
    message: RenderedTranslatedMessage,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &impl Translatable) -> Self {
        Self {
            kind,
            trace: Trace::default(),
            message: message.render(),
        }
    }

    pub fn get_message(&self) -> &RenderedTranslatedMessage {
        &self.message
    }

    pub fn get_error_kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    pub fn stack_trace(&self) -> &Trace {
        &self.trace
    }

    pub fn division_by_zero() -> Self {
        Self::new(
            ErrorKind::DivisionByZero,
            &TranslatedMessage::new(
                "engine.evaluate.error.division_by_zero",
                translation_resolver,
            ),
        )
    }

    pub fn incompatible_matrix_dimensions(
        left: Vec<usize>,
        right: Vec<usize>,
    ) -> Self {
        Self::new(
            ErrorKind::IncompatibleMatrixDimensions,
            &TranslatedMessage::new(
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

    pub fn incompatible_tensor_shapes(left: &[usize], right: &[usize]) -> Self {
        Self::new(
            ErrorKind::IncompatibleVectorDimensions,
            &TranslatedMessage::new(
                "engine.evaluate.error.incompatible_tensor_shapes",
                translation_resolver,
            )
            .key(
                "left",
                left.iter()
                    .map(|dim| dim.to_string())
                    .collect::<Vec<_>>()
                    .join("x"),
            )
            .key(
                "right",
                right
                    .iter()
                    .map(|dim| dim.to_string())
                    .collect::<Vec<_>>()
                    .join("x"),
            ),
        )
    }

    pub fn incompatible_vector_dimensions(left: usize, right: usize) -> Self {
        Self::new(
            ErrorKind::IncompatibleVectorDimensions,
            &TranslatedMessage::new(
                "engine.evaluate.error.incompatible_vector_dimensions",
                translation_resolver,
            )
            .key("left", left)
            .key("right", right),
        )
    }

    pub fn invalid_node(node_type: NodeType) -> Self {
        Self::new(
            ErrorKind::InvalidNode,
            &TranslatedMessage::new(
                "engine.evaluate.error.invalid_node",
                translation_resolver,
            )
            .key("node_type", node_type.to_string()),
        )
    }

    pub fn invalid_parameter_type(
        name: &str,
        expected: HashSet<NodeType>,
        received: NodeType,
    ) -> Self {
        Self::new(
            ErrorKind::InvalidParameterType,
            &TranslatedMessage::new(
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
            .key("received", received.to_string()),
        )
    }

    pub fn invalid_repeatable_parameter_count(
        name: &str,
        min: usize,
        max: Option<usize>,
        received: usize,
    ) -> Self {
        Self::new(
            ErrorKind::InvalidRepeatableParameterCount,
            &TranslatedMessage::new(
                "engine.evaluate.error.invalid_repeatable_parameter_count",
                translation_resolver,
            )
            .key("name", name)
            .key("min", min)
            .key("max", format!("{:?}", max))
            .key("received", received),
        )
    }

    pub fn missing_parameter(name: impl IntoValue) -> Self {
        Self::new(
            ErrorKind::MissingParameter,
            &TranslatedMessage::new(
                "engine.evaluate.error.missing_parameter",
                translation_resolver,
            )
            .key("name", name),
        )
    }

    pub fn runtime_error(message: impl IntoValue) -> Self {
        Self::new(
            ErrorKind::RuntimeError,
            &TranslatedMessage::new(
                "engine.evaluate.error.runtime_error",
                translation_resolver,
            )
            .key("message", message),
        )
    }

    pub fn too_many_parameters(count: usize) -> Self {
        Self::new(
            ErrorKind::TooManyParameters,
            &TranslatedMessage::new(
                "engine.evaluate.error.too_many_parameters",
                translation_resolver,
            )
            .key("count", count),
        )
    }

    pub fn unknown_symbol<T: ToString>(name: T) -> Self {
        Self::new(
            ErrorKind::UnknownSymbol,
            &TranslatedMessage::new(
                "engine.evaluate.error.unknown_symbol",
                translation_resolver,
            )
            .key("name", name.to_string()),
        )
    }

    pub fn unsupported_datatype(node_kind: NodeType) -> Self {
        Self::new(
            ErrorKind::UnsupportedDataType,
            &TranslatedMessage::new(
                "engine.evaluate.error.unsupported_datatype",
                translation_resolver,
            )
            .key("node_kind", node_kind.to_string()),
        )
    }

    pub fn unsupported_operation() -> Self {
        Self::new(
            ErrorKind::UnsupportedOperation,
            &TranslatedMessage::new(
                "engine.evaluate.error.unsupported_operation",
                translation_resolver,
            ),
        )
    }

    pub fn unreachable() -> Self {
        Self::new(
            ErrorKind::RuntimeError,
            &TranslatedMessage::new(
                "engine.evaluate.error.unreachable",
                translation_resolver,
            ),
        )
    }

    pub fn overflow() -> Self {
        Self::new(
            ErrorKind::RuntimeError,
            &TranslatedMessage::new(
                "engine.evaluate.error.overflow",
                translation_resolver,
            ),
        )
    }
}

impl AsRef<Trace> for Error {
    fn as_ref(&self) -> &Trace {
        &self.trace
    }
}

impl AsMut<Trace> for Error {
    fn as_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

pub trait MapToEvaluatorError {
    type Output;

    fn map_to_error_kind(
        self,
        error_kind: ErrorKind,
    ) -> Result<Self::Output, Error>;
}

impl<T, E: Translatable + std::error::Error> MapToEvaluatorError
    for Result<T, E>
{
    type Output = T;

    fn map_to_error_kind(
        self,
        error_kind: ErrorKind,
    ) -> Result<Self::Output, Error> {
        self.map_err(|err| Error::new(error_kind, &err))
    }
}
