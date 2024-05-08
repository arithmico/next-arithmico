use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_negate(value: &Node) -> Result<Node, EvaluationError> {
    let inner_value = value.evaluate()?;
    match inner_value {
        Node::Number { value } => Ok(Node::Number { value: -value }),
        _ => Err(EvaluationError::UnsupportedOperation),
    }
}
