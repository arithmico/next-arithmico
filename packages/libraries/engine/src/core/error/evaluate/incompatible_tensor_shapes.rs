use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn incompatible_tensor_shapes(left: usize, right: usize) -> Self {
        Self::new(
            EvaluateNodeErrorKind::IncompatibleVectorDimensions,
            TranslatedMessage::new(
                "engine.evaluate.error.incompatible_tensor_shapes",
                translation_resolver,
            )
            .key("left", left)
            .key("right", right),
        )
    }
}
