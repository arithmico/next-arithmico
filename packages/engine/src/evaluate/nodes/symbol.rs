use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_symbol(
    name: &String,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    context
        .lookup(name)
        .ok_or_else(|| NodeEvaluationError::UnknownSymbol)
}
