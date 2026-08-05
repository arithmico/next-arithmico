use translate::IntoValue;
use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn missing_parameter(name: impl IntoValue) -> Self {
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
