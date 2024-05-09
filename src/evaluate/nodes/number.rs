use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_number(
    value: &f64,
    _context: &Context,
) -> Result<Node, EvaluationError> {
    Ok(Node::Number {
        value: f64::clone(value),
    })
}
