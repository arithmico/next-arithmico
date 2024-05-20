use std::iter::zip;

use crate::{
    core::{context::Context, evaluate::NodeEvaluationError, node::Node},
    utils::vector_utils::{get_tensor_dimensions, get_tensor_rank},
};

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
        result = multiply_nodes(&result, &evaluated_current_node, context)?;
    }

    Ok(result)
}

fn multiply_nodes(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    match (left, right) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) if cfg!(feature = "operator_product_number_number") => {
            Ok(Node::Number {
                value: left_value * right_value,
            })
        }
        (Node::Vector { .. }, Node::Vector { .. }) => {
            multiply_tensors(left, right, context)
        }
        (Node::Number { value }, Node::Vector { values })
        | (Node::Vector { values }, Node::Number { value })
            if cfg!(feature = "operator_product_number_vector") =>
        {
            let mut new_values = Vec::<Node>::new();
            for vector_value in values {
                new_values.push(
                    Node::Product {
                        values: vec![
                            Node::Number { value: *value },
                            vector_value.clone(),
                        ],
                    }
                    .evaluate(context)?,
                );
            }
            Ok(Node::Vector { values: new_values })
        }
        _ => Err(NodeEvaluationError::UnsupportedOperation),
    }
}

fn multiply_tensors(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    match (left, right) {
        (
            Node::Vector {
                values: left_values,
            },
            Node::Vector {
                values: right_values,
            },
        ) => match (get_tensor_rank(left), get_tensor_rank(right)) {
            (Some(1), Some(1))
                if cfg!(feature = "operator_product_vector_vector") =>
            {
                if get_tensor_dimensions(left) != get_tensor_dimensions(right) {
                    return Err(NodeEvaluationError::ArithmeticError(
                        "Can not multiply vectors with different dimensions"
                            .into(),
                    ));
                }
                Node::Sum {
                    values: zip(left_values, right_values)
                        .map(|(left, right)| Node::Product {
                            values: vec![left.clone(), right.clone()],
                        })
                        .collect(),
                }
                .evaluate(context)
            }
            _ => Err(NodeEvaluationError::UnsupportedOperation),
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_product_numbers() {
        let context = Context::default();
        assert_eq!(
            Node::Product {
                values: vec![
                    Node::Number { value: 3.0 },
                    Node::Number { value: 2.0 }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 6.0 }
        )
    }

    #[test]
    fn evaluate_product_vectors() {
        let context = Context::default();
        assert_eq!(
            Node::Product {
                values: vec![
                    Node::Vector {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                            Node::Number { value: 3.0 }
                        ]
                    },
                    Node::Vector {
                        values: vec![
                            Node::Number { value: 3.0 },
                            Node::Number { value: 2.0 },
                            Node::Number { value: 1.0 }
                        ]
                    }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 10.0 }
        )
    }

    #[test]
    fn evaluate_product_number_vector() {
        let context = Context::default();
        assert_eq!(
            Node::Product {
                values: vec![
                    Node::Number { value: 2.0 },
                    Node::Vector {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                            Node::Number { value: 3.0 }
                        ]
                    },
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 2.0 },
                    Node::Number { value: 4.0 },
                    Node::Number { value: 6.0 }
                ]
            },
        )
    }

    #[test]
    fn evaluate_product_vector_number() {
        let context = Context::default();
        assert_eq!(
            Node::Product {
                values: vec![
                    Node::Vector {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                            Node::Number { value: 3.0 }
                        ]
                    },
                    Node::Number { value: 2.0 },
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 2.0 },
                    Node::Number { value: 4.0 },
                    Node::Number { value: 6.0 }
                ]
            },
        )
    }
}
