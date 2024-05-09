use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_division(
    dividend: &Node,
    divisor: &Node,
) -> Result<Node, EvaluationError> {
    match (dividend, divisor) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) => Ok(Node::Number {
            value: left_value / right_value,
        }),
        _ => return Err(EvaluationError::UnsupportedOperation),
    }
}
