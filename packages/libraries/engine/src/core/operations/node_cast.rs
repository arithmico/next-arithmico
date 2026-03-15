use crate::{Node, core::EvaluateNodeError};

pub trait NodeCast: Sized {
    fn try_from_node(node: &Node) -> Result<Self, EvaluateNodeError>;
}