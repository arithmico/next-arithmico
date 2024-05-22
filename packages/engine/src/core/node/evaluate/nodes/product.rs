use std::iter::zip;

use crate::{
    core::{
        context::Context,
        node::{
            evaluate::NodeEvaluationError, Node, Number, Product, Sum, Tensor,
        },
    },
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
            Node::Number(Number { value: left_value }),
            Node::Number(Number { value: right_value }),
        ) if cfg!(feature = "operator_product_number_number") => {
            Ok(Number::new(left_value * right_value).into())
        }
        (Node::Tensor { .. }, Node::Tensor { .. }) => {
            multiply_tensors(left, right, context)
        }
        (
            Node::Number(Number { value }),
            Node::Tensor(Tensor { elements: values }),
        )
        | (
            Node::Tensor(Tensor { elements: values }),
            Node::Number(Number { value }),
        ) if cfg!(feature = "operator_product_number_vector") => {
            let mut new_values = Vec::<Node>::new();
            for vector_value in values {
                new_values.push(
                    Node::from(Product::new(vec![
                        Number::new(*value).into(),
                        vector_value.clone(),
                    ]))
                    .evaluate(context)?,
                );
            }
            Ok(Tensor::new(new_values).into())
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
            Node::Tensor(Tensor {
                elements: left_values,
            }),
            Node::Tensor(Tensor {
                elements: right_values,
            }),
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
                Node::from(Sum::new(
                    zip(left_values, right_values)
                        .map(|(left, right)| {
                            Product::new(vec![left.clone(), right.clone()])
                                .into()
                        })
                        .collect(),
                ))
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
            Node::from(Product::new(vec![
                Number::new(3.0).into(),
                Number::new(2.0).into()
            ]))
            .evaluate(&context)
            .unwrap(),
            Number::new(6.0).into()
        )
    }

    #[test]
    fn evaluate_product_vectors() {
        let context = Context::default();
        assert_eq!(
            Node::from(Product::new(vec![
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(3.0).into(),
                    Number::new(2.0).into(),
                    Number::new(1.0).into(),
                ])
                .into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Number::new(10.0).into()
        )
    }

    #[test]
    fn evaluate_product_number_vector() {
        let context = Context::default();
        assert_eq!(
            Node::from(Product::new(vec![
                Number::new(2.0).into(),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Number::new(2.0).into(),
                Number::new(4.0).into(),
                Number::new(6.0).into(),
            ])
            .into(),
        )
    }

    #[test]
    fn evaluate_product_vector_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Product::new(vec![
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
                Number::new(2.0).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Number::new(2.0).into(),
                Number::new(4.0).into(),
                Number::new(6.0).into(),
            ])
            .into(),
        )
    }
}
