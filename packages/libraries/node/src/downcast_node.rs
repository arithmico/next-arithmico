use thiserror::Error;
use trace::Trace;
use translate_core::{Translatable, TranslatedMessage};

use crate::{
    Node, NodeType, Tensor, translation_provider::translation_resolver,
};

#[derive(Debug, Clone, Error)]
#[error("Failed to downcast Node. Expected {expected} but received {received}")]
pub struct DowncastNodeError {
    pub expected: NodeType,
    pub received: NodeType,
    pub trace: Trace,
}

impl Translatable for DowncastNodeError {
    fn translate(
        &self,
        language: translate_core::Language,
    ) -> Result<String, translate_core::TranslationError> {
        let message = TranslatedMessage::new(
            "error.unexpected_node_type",
            translation_resolver,
        )
        // TODO: translate node types
        .key("expected", self.expected.to_string())
        .key("received", self.received.to_string());

        message.translate(language)
    }
}

pub trait DowncastNodeVec {
    fn downcast<'a, T>(&'a self) -> Result<Vec<&'a T>, DowncastNodeError>
    where
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>;
}

impl DowncastNodeVec for Vec<Node> {
    fn downcast<'a, T>(&'a self) -> Result<Vec<&'a T>, DowncastNodeError>
    where
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>,
    {
        self.iter().map(|node| node.try_into()).collect()
    }
}

impl DowncastNodeVec for Tensor {
    fn downcast<'a, T>(&'a self) -> Result<Vec<&'a T>, DowncastNodeError>
    where
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>,
    {
        self.elements.iter().map(|node| node.try_into()).collect()
    }
}
