use crate::{Node, Symbol, core::{EvaluateNodeError, NodeCast}};

impl NodeCast for Symbol {
    fn downcast(node: &Node) -> Result<&Self, EvaluateNodeError> {
        match node {
            Node::Symbol(symbol) => Ok(symbol),
            _ => Err(EvaluateNodeError::runtime_error("expected symbol")
                .with_tracable(node)),
        }
    }
}