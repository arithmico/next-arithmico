use std::collections::HashSet;

use translate_core::TranslatedMessage;

use crate::core::{NodeType, translation_resolver};

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn invalid_parameter_type(
        name: String,
        expected: HashSet<NodeType>,
        received: NodeType,
    ) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidParameterType,
            TranslatedMessage::new(
                "engine.evaluate.error.invalid_parameter_type",
                translation_resolver,
            )
            .key("name", name)
            .key(
                "expected",
                expected
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .key("received", received),
        )
    }
}
