use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_product(
    values: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    if values.len() < 2 {
        return Err(NodeEvaluationError::InvalidNumberOfValues);
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
                    value: left_value * right_value,
                }
            }
            _ => return Err(NodeEvaluationError::UnsupportedOperation),
        }
    }

    Ok(result)
}
