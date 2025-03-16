use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn unknown_symbol<T: ToString>(name: T) -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnknownSymbol,
            TranslatedMessage::new(
                "engine.evaluate.error.unknown_symbol",
                translation_resolver,
            )
            .key("name", name),
        )
    }
}
