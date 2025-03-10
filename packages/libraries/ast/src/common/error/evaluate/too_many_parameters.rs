use translate_core::TranslatedMessage;

use crate::common::translation_provider::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn too_many_parameters(count: usize) -> Self {
        Self::new(
            EvaluateNodeErrorKind::TooManyParameters,
            TranslatedMessage::new(
                "engine.evaluate.error.too_many_parameters",
                translation_resolver,
            )
            .key("count", count),
        )
    }
}
