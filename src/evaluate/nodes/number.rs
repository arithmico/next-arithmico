use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_number(
    value: &f64,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(Node::Number {
        value: f64::clone(value),
    })
}
