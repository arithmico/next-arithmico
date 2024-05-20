use crate::{
    context::Context, evaluate::NodeEvaluationError, node::Node,
    utils::vector_utils::get_tensor_dimensions,
};

pub fn evaluate_sum(
    values: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    if values.len() < 2 {
        return Err(NodeEvaluationError::InvalidNumberOfValues);
    }
    let mut result = values[0].evaluate(context)?;
    for current_node in values[1..].iter() {
        let evaluated_current_node = current_node.evaluate(context)?;
        result = add_nodes(&result, &evaluated_current_node, context)?;
    }

    Ok(result)
}

fn add_nodes(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    match (left, right) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) if cfg!(feature = "operator_sum_number_number") => Ok(Node::Number {
            value: left_value + right_value,
        }),
        (
            Node::Vector {
                values: left_values,
            },
            Node::Vector {
                values: right_values,
            },
        ) if cfg!(feature = "operator_sum_vector_vector") => {
            if get_tensor_dimensions(left) != get_tensor_dimensions(right) {
                return Err(NodeEvaluationError::ArithmeticError(
                    "Unable to perform vector addition due to mismatching dimensions".into(),
                ));
            }

            Node::Vector {
                values: left_values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| Node::Sum {
                        values: vec![
                            value.clone(),
                            right_values.get(index).unwrap().clone(),
                        ],
                    })
                    .collect(),
            }
            .evaluate(context)
        }
        _ => return Err(NodeEvaluationError::UnsupportedOperation),
    }
}
