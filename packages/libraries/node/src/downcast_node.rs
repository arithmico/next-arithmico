use thiserror::Error;
use trace::Trace;
use translate_core::{Translatable, TranslatedMessage};

use crate::{
    GetStaticNodeType, Node, NodeType,
    translation_provider::translation_resolver,
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

pub trait DowncastNode: Sized + GetStaticNodeType {
    fn downcast_node(node: &Node) -> Result<&Self, DowncastNodeError>;
}

impl Node {
    pub fn downcast<T: DowncastNode>(&self) -> Result<&T, DowncastNodeError> {
        T::downcast_node(self)
    }
}

pub trait DowncastNodeVec {
    fn downcast<T: DowncastNode>(&self) -> Result<Vec<&T>, DowncastNodeError>;
}

impl DowncastNodeVec for Vec<Node> {
    fn downcast<T: DowncastNode>(&self) -> Result<Vec<&T>, DowncastNodeError> {
        self.iter().map(|node| T::downcast_node(node)).collect()
    }
}
