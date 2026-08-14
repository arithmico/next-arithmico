use translate_core::{
    Language, Translatable, TranslatedMessage, TranslationError,
};

use crate::translation_provider::translation_resolver;

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
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
    UnexpectedNodeType,
}

impl Translatable for ErrorKind {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        let translation_id = match self {
            ErrorKind::UnsupportedOperation => {
                "engine.evaluate.error.unsupported_operation.error_kind"
            }
            ErrorKind::UnsupportedDataType => {
                "engine.evaluate.error.unsupported_datatype.error_kind"
            }
            ErrorKind::UnknownSymbol => {
                "engine.evaluate.error.unknown_symbol.error_kind"
            }
            ErrorKind::RuntimeError => {
                "engine.evaluate.error.runtime_error.error_kind"
            }
            ErrorKind::InvalidNode => {
                "engine.evaluate.error.invalid_node.error_kind"
            }
            ErrorKind::IncompatibleVectorDimensions => {
                "engine.evaluate.error.incompatible_vector_dimensions.error_kind"
            }
            ErrorKind::IncompatibleMatrixDimensions => {
                "engine.evaluate.error.incompatible_matrix_dimensions.error_kind"
            }
            ErrorKind::DivisionByZero => {
                "engine.evaluate.error.division_by_zero.error_kind"
            }
            ErrorKind::MissingParameter => {
                "engine.evaluate.error.missing_parameter.error_kind"
            }
            ErrorKind::InvalidRepeatableParameterCount => {
                "engine.evaluate.error.invalid_repeatable_parameter_count.error_kind"
            }
            ErrorKind::InvalidParameterType => {
                "engine.evaluate.error.invalid_parameter_type.error_kind"
            }
            ErrorKind::TooManyParameters => {
                "engine.evaluate.error.too_many_parameters.error_kind"
            }
            ErrorKind::InvalidParameterValue => {
                "engine.evaluate.error.invalid_parameter_value.error_kind"
            }
            ErrorKind::UnexpectedNodeType => {
                "engine.evaluate.error.unexpected_node_type.error_kind"
            }
        };

        TranslatedMessage::new(translation_id, translation_resolver)
            .translate(language)
    }
}
