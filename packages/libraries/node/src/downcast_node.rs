use thiserror::Error;
use trace::Trace;

use crate::{GetStaticNodeType, Node, NodeType};

#[derive(Debug, Clone, Error)]
#[error("Failed to downcast Node. Expected {expected} but received {received}")]
pub struct DowncastNodeError {
    pub expected: NodeType,
    pub received: NodeType,
    pub trace: Trace,
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
