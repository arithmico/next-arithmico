use node::{Node, Number};

use crate::core::{EvaluateNodeError, NodeCast};

impl NodeCast for Number {
    fn downcast(node: &Node) -> Result<&Self, EvaluateNodeError> {
        match node {
            Node::Number(number) => Ok(number),
            _ => Err(EvaluateNodeError::runtime_error("expected number")
                .with_tracable(node)),
        }
    }
}
