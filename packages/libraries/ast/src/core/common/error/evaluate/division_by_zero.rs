use translate_core::TranslatedMessage;

use crate::core::common::translation_provider::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn division_by_zero() -> Self {
        Self::new(
            EvaluateNodeErrorKind::DivisionByZero,
            TranslatedMessage::new(
                "engine.evaluate.error.division_by_zero",
                translation_resolver,
            ),
        )
    }
}
