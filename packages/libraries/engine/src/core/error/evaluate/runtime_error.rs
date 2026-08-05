use translate::IntoValue;
use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn runtime_error(message: impl IntoValue) -> Self {
        Self::new(
            EvaluateNodeErrorKind::RuntimeError,
            TranslatedMessage::new(
                "engine.evaluate.error.runtime_error",
                translation_resolver,
            )
            .key("message", message),
        )
    }
}
