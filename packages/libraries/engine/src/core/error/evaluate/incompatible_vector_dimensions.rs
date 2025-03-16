use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn incompatible_vector_dimensions(left: usize, right: usize) -> Self {
        Self::new(
            EvaluateNodeErrorKind::IncompatibleVectorDimensions,
            TranslatedMessage::new(
                "engine.evaluate.error.incompatible_vector_dimensions",
                translation_resolver,
            )
            .key("left", left)
            .key("right", right),
        )
    }
}
