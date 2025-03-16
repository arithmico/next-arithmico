use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn unsupported_operation() -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnsupportedOperation,
            TranslatedMessage::new(
                "engine.evaluate.error.unsupported_operation",
                translation_resolver,
            ),
        )
    }
}
