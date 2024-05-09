use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_symbol(
    name: &String,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    match context.lookup(name) {
        Some(node) => Ok(node.clone()),
        None => Err(NodeEvaluationError::UnknownSymbol),
    }
}
