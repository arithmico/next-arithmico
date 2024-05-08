use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_number(value: &f64) -> Result<Node, EvaluationError> {
    Ok(Node::Number {
        value: f64::clone(value),
    })
}
