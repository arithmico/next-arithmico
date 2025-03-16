use crate::core::{
    common::translation_provider::translation_resolver, NodeType,
};
use translate_core::TranslatedMessage;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

impl EvaluateNodeError {
    pub fn unsupported_datatype(node_kind: NodeType) -> Self {
        Self::new(
            EvaluateNodeErrorKind::UnsupportedDataType,
            TranslatedMessage::new(
                "engine.evaluate.error.unsupported_datatype",
                translation_resolver,
            )
            .key("node_kind", node_kind),
        )
    }
}
