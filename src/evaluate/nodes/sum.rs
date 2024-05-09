use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_sum(
    values: &Vec<Node>,
    context: &Context,
) -> Result<Node, EvaluationError> {
    if values.len() < 2 {
        return Err(EvaluationError::InvalidNumberOfValues);
    }
    let mut result = values[0].evaluate(context)?;
    for current_node in values[1..].iter() {
        let evaluated_current_node = current_node.evaluate(context)?;
        match (result, evaluated_current_node) {
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
