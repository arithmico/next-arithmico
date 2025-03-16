use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn incompatible_matrix_dimensions(
        left: Vec<usize>,
        right: Vec<usize>,
    ) -> Self {
        Self::new(
            EvaluateNodeErrorKind::IncompatibleMatrixDimensions,
            TranslatedMessage::new(
                "engine.evaluate.error.incompatible_matrix_dimensions",
                translation_resolver,
            )
            .key(
                "left",
                left.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("x"),
            )
            .key(
                "right",
                right
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("x"),
            ),
        )
    }
}
