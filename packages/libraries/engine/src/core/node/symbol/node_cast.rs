use crate::{Node, Symbol, core::{EvaluateNodeError, NodeCast}};

impl NodeCast for Symbol {
    fn try_from_node(node: &Node) -> Result<Self, EvaluateNodeError> {
        match node {
            Node::Symbol(symbol) => Ok(symbol.clone()),
            _ => Err(EvaluateNodeError::runtime_error("expected symbol")
                .with_tracable(node)),
        }
    }
}