use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn invalid_repeatable_parameter_count(
        name: String,
        min: usize,
        max: Option<usize>,
        received: usize,
    ) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidRepeatableParameterCount,
            TranslatedMessage::new(
                "engine.evaluate.error.invalid_repeatable_parameter_count",
                translation_resolver,
            )
            .key("name", name)
            .key("min", min)
            .key("max", format!("{:?}", max))
            .key("received", received),
        )
    }
}
