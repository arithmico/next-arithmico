use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_sum(nodes: &Vec<Node>) -> Result<Node, EvaluationError> {
    if nodes.len() < 2 {
        return Err(EvaluationError::InvalidNumberOfValues);
    }
    let mut result = nodes[0].evaluate()?;
    for current in nodes[1..].iter() {
        match (result, current) {
            (
                Node::Number { value: left_value },
                Node::Number { value: right_value },
            ) => {
                result = Node::Number {
                    value: left_value + right_value,
                }
            }
            _ => return Err(EvaluationError::UnsupportedOperation),
        }
    }

    Ok(result)
}
