use translate_core::TranslatedMessage;

use crate::core::common::translation_provider::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
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
}
