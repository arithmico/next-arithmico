use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
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
}
