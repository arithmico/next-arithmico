use crate::NodeType;
use translate_core::TranslatedMessage;

use crate::common::translation_provider::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn invalid_node(node_type: NodeType) -> Self {
        Self::new(
            EvaluateNodeErrorKind::InvalidNode,
            TranslatedMessage::new(
                "engine.evaluate.error.invalid_node",
                translation_resolver,
            )
            .key("node_type", node_type),
        )
    }
}
