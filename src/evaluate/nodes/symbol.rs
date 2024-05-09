use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_symbol(
    name: &String,
    context: &Context,
) -> Result<Node, EvaluationError> {
    match context.lookup(name) {
        Some(node) => Ok(node.clone()),
        None => Err(EvaluationError::UnknownSymbol),
    }
}
