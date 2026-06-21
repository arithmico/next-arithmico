use node::Node;

use crate::core::EvaluateNodeError;

pub trait NodeCast: Sized {
    fn downcast(node: &Node) -> Result<&Self, EvaluateNodeError>;
}

#[allow(dead_code)]
pub trait DowncastNode {
    fn downcast<T: NodeCast>(&self) -> Result<&T, EvaluateNodeError>;
}

impl DowncastNode for Node {
    fn downcast<T: NodeCast>(&self) -> Result<&T, EvaluateNodeError> {
        T::downcast(self)
    }
}
