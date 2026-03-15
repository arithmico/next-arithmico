use crate::{Node, Number, core::{EvaluateNodeError, NodeCast}};


impl NodeCast for Number {
    fn try_from_node(node: &Node) -> Result<Self, EvaluateNodeError> {
        match node {
            Node::Number(number) => Ok(number.clone()),
            _ => Err(EvaluateNodeError::runtime_error("expected number")
                .with_tracable(node)),
        }
    }
}