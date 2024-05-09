use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_product(values: &Vec<Node>) -> Result<Node, EvaluationError> {
    if values.len() < 2 {
        return Err(EvaluationError::InvalidNumberOfValues);
    }
    let mut result = values[0].evaluate()?;
    for current in values[1..].iter() {
        match (result, current) {
            (
                Node::Number { value: left_value },
                Node::Number { value: right_value },
            ) => {
                result = Node::Number {
                    value: left_value * right_value,
                }
            }
            _ => return Err(EvaluationError::UnsupportedOperation),
        }
    }

    Ok(result)
}
