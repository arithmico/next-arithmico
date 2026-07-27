use translate_core::{Translatable, TranslatedMessage};

use crate::core::translation_resolver;

#[derive(Debug, PartialEq, Clone)]
pub enum EvaluateNodeErrorKind {
    UnsupportedOperation,
    UnsupportedDataType,
    UnknownSymbol,
    RuntimeError,
    InvalidNode,
    IncompatibleVectorDimensions,
    IncompatibleMatrixDimensions,
    DivisionByZero,
    MissingParameter,
    InvalidRepeatableParameterCount,
    InvalidParameterType,
    TooManyParameters,
    InvalidParameterValue,
}

impl Translatable for EvaluateNodeErrorKind {
    fn translate(
        &self,
        language: translate_core::Language,
    ) -> Result<String, translate_core::TranslationError> {
        let translation_id = match self {
            EvaluateNodeErrorKind::UnsupportedOperation => {
                "engine.evaluate.error.unsupported_operation.error_kind"
            }
            EvaluateNodeErrorKind::UnsupportedDataType => {
                "engine.evaluate.error.unsupported_datatype.error_kind"
            }
            EvaluateNodeErrorKind::UnknownSymbol => {
                "engine.evaluate.error.unknown_symbol.error_kind"
            }
            EvaluateNodeErrorKind::RuntimeError => {
                "engine.evaluate.error.runtime_error.error_kind"
            }
            EvaluateNodeErrorKind::InvalidNode => {
                "engine.evaluate.error.invalid_node.error_kind"
            }
            EvaluateNodeErrorKind::IncompatibleVectorDimensions => {
                "engine.evaluate.error.incompatible_vector_dimensions.error_kind"
            }
            EvaluateNodeErrorKind::IncompatibleMatrixDimensions => {
                "engine.evaluate.error.incompatible_matrix_dimensions.error_kind"
            }
            EvaluateNodeErrorKind::DivisionByZero => {
                "engine.evaluate.error.division_by_zero.error_kind"
            }
            EvaluateNodeErrorKind::MissingParameter => {
                "engine.evaluate.error.missing_parameter.error_kind"
            }
            EvaluateNodeErrorKind::InvalidRepeatableParameterCount => {
                "engine.evaluate.error.invalid_repeatable_parameter_count.error_kind"
            }
            EvaluateNodeErrorKind::InvalidParameterType => {
                "engine.evaluate.error.invalid_parameter_type.error_kind"
            }
            EvaluateNodeErrorKind::TooManyParameters => {
                "engine.evaluate.error.too_many_parameters.error_kind"
            }
            EvaluateNodeErrorKind::InvalidParameterValue => {
                "engine.evaluate.error.invalid_parameter_value.error_kind"
            }
        };

        TranslatedMessage::new(translation_id, translation_resolver)
            .translate(language)
    }
}
