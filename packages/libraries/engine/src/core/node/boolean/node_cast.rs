use node::{Boolean, Node};

use crate::core::{EvaluateNodeError, NodeCast};

impl NodeCast for Boolean {
    fn downcast(node: &Node) -> Result<&Self, EvaluateNodeError> {
        match node {
            Node::Boolean(bool) => Ok(bool),
            _ => Err(EvaluateNodeError::runtime_error("expected boolean")
                .with_tracable(node)),
        }
    }
}
