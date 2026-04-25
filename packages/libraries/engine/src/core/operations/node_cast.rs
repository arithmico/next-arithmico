use crate::{core::EvaluateNodeError, Node};

pub trait NodeCast: Sized {
    fn downcast(node: &Node) -> Result<&Self, EvaluateNodeError>;
}

impl Node {
    pub fn downcast<T: NodeCast>(&self) -> Result<&T, EvaluateNodeError> {
        T::downcast(self)
    }
}
