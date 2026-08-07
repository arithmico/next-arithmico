pub use error_kind::*;
use math_utils::{FindRootsError, IntegrationError};
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

impl From<node_validator::Error> for EvaluateNodeError {
    fn from(value: node_validator::Error) -> Self {
        let mut err = Self::new(
            EvaluateNodeErrorKind::InvalidParameterValue,
            value.message,
        );
        err.trace = value.trace;
        err
    }
}

impl From<FindRootsError> for EvaluateNodeError {
    fn from(value: FindRootsError) -> Self {
        match value {
            FindRootsError::NonFiniteBoundary { name, value } => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.find_roots.non_finite_boundary",
                        translation_resolver
                    )
                    .key("name", name)
                .key("value", value))
            }
            FindRootsError::InvalidInterval { start, end } => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.find_roots.invalid_interval",
                        translation_resolver,
                    )
                .key("start", start)
                .key("end", end)
            )
            }
            FindRootsError::NonFiniteFunctionValue { x, value } => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                    "engine.api.error.generic_runtime_error.find_roots.non_finite_function_value",
                        translation_resolver,
                    )
                .key("x", x)
                .key("value", value)
            )
            }
            FindRootsError::EncloseZero { a, b, .. } => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                    "engine.api.error.generic_runtime_error.find_roots.enclose_zero",
                        translation_resolver,
                    )
                .key("a", a)
                .key("b", b)
            )
            }
        }
    }
}

impl From<IntegrationError> for EvaluateNodeError {
    fn from(value: IntegrationError) -> Self {
        match value {
            IntegrationError::InvalidTolerance => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.nintegrate.invalid_tolerance",
                        translation_resolver,
                    ),
                )
            }
            IntegrationError::SubdivisionLimitReached => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.nintegrate.subdivision_limit_reached",
                        translation_resolver,
                    ),
                )
            }
            IntegrationError::RoundoffError => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.nintegrate.roundoff_error",
                        translation_resolver,
                    ),
                )
            }
            IntegrationError::BadIntegrandBehavior => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.nintegrate.bad_integrand_behavior",
                        translation_resolver,
                    ),
                )
            }
            IntegrationError::ExtrapolationFailed => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.nintegrate.extrapolation_failed",
                        translation_resolver,
                    ),
                )
            }
            IntegrationError::ProbablyDivergent => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                        "engine.api.error.generic_runtime_error.nintegrate.probably_divergent",
                        translation_resolver,
                    ),
                )
            }
            IntegrationError::NonFiniteIntegrand { x, value } => {
                Self::new(
                    EvaluateNodeErrorKind::RuntimeError,
                    TranslatedMessage::new(
                "engine.api.error.generic_runtime_error.nintegrate.non_finite_integrand",
                translation_resolver
                    )
                    .key("x", x)
                    .key("value", value))
            }
        }
    }
}
