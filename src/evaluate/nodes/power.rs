use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_power(
    base: &Node,
    exponent: &Node,
) -> Result<Node, EvaluationError> {
    match (base, exponent) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) => Ok(Node::Number {
            value: left_value.powf(*right_value),
        }),
        _ => return Err(EvaluationError::UnsupportedOperation),
    }
}
