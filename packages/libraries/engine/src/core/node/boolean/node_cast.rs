use crate::{Boolean, Node, core::{EvaluateNodeError, NodeCast}};

impl NodeCast for Boolean {
    fn try_from_node(node: &Node) -> Result<Self, EvaluateNodeError> {
        match node {
            Node::Boolean(bool) => Ok(bool.clone()),
            _ => Err(EvaluateNodeError::runtime_error("expected boolean")
                .with_tracable(node)),
        }
    }
}