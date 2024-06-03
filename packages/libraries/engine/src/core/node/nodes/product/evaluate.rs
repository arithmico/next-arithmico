use std::{iter::zip, rc::Rc};

use crate::{
    core::{context::Context, node::*},
    utils::tensor::index_utils::convert_to_outer_index,
};

impl EvaluateNode for Product {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        if self.values.len() < 2 {
            return Err(NodeError::RuntimeError(String::from(
                "invalid number of operands",
            )));
        }
        let mut result = self.values[0].evaluate(context)?;
        for current_node in self.values[1..].iter() {
            let evaluated_current_node = current_node.evaluate(context)?;
            result = multiply_nodes(&result, &evaluated_current_node, context)?;
        }
        Ok(result)
    }
}

fn multiply_nodes(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, NodeError> {
    match (left, right) {
        (
            Node::Number(Number { value: left_value }),
            Node::Number(Number { value: right_value }),
        ) if cfg!(feature = "operator_product_number_number") => {
            Ok(Number::new(left_value * right_value).into())
        }
        (Node::Tensor(left), Node::Tensor(right)) => {
            multiply_tensors(left, right, context)
        }
        (
            Node::Number(Number { value }),
            Node::Tensor(Tensor {
                elements: values, ..
            }),
        )
        | (
            Node::Tensor(Tensor {
                elements: values, ..
            }),
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
        _ => Err(NodeError::UnsupportedOperation),
    }
}

fn multiply_tensors(
    left: &Tensor,
    right: &Tensor,
    context: &Context,
) -> Result<Node, NodeError> {
    let left_rank = left.get_rank();
    let right_rank = right.get_rank();
    match (left_rank, right_rank) {
        (1, 1) if cfg!(feature = "operator_product_vector_vector") => {
            if left.shape != right.shape {
                return Err(NodeError::RuntimeError(
                    "can not multiply vectors with different dimensions".into(),
                ));
            }
            Node::from(Sum::new(
                zip(left.elements.iter(), right.elements.iter())
                    .map(|(left, right)| {
                        Product::new(vec![left.clone(), right.clone()]).into()
                    })
                    .collect(),
            ))
            .evaluate(context)
        }
        (2, 2) if cfg!(feature = "operator_product_matrix_matrix") => {
            match (left.shape.get(1), right.shape.get(0)) {
                (Some(x), Some(y)) if x == y => {
                    let left = Rc::new(left.clone());
                    let right = Rc::new(right.clone());
                    let result_shape = vec![
                        *left.shape.get(0).unwrap(),
                        *right.shape.get(1).unwrap(),
                    ];
                    let elements: Vec<_> = {
                        let left = left.clone();
                        let right = right.clone();
                        let dim0 = *left.shape.get(0).unwrap();
                        let dim1 = *right.shape.get(1).unwrap();
                        let result_shape = result_shape.clone();
                        (0usize..(dim0 * dim1))
                            .map(move |inner_index| {
                                let outer_index = convert_to_outer_index(
                                    &result_shape,
                                    inner_index,
                                )
                                .unwrap();
                                let i = *outer_index.get(0).unwrap();
                                let k = *outer_index.get(1).unwrap();
                                let left = left.clone();
                                let right = right.clone();
                                Node::from(Sum::new(
                                    (0usize..*x)
                                        .map(move |j| {
                                            Node::from(Product::new(vec![
                                                left.get_element(&vec![i, j])
                                                    .unwrap()
                                                    .clone(),
                                                right
                                                    .get_element(&vec![j, k])
                                                    .unwrap()
                                                    .clone(),
                                            ]))
                                        })
                                        .collect(),
                                ))
                            })
                            .collect()
                    };
                    Node::from(
                        Tensor::new_with_shape(elements, result_shape).unwrap(),
                    )
                    .evaluate(context)
                }
                _ => Err(NodeError::RuntimeError(
                    "Can not multiply matrices with icompatible dimensions"
                        .into(),
                )),
            }
        }
        _ => Err(NodeError::UnsupportedOperation),
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

    #[test]
    fn evaluate_product_matrix_matrix() {
        let context = Context::default();
        assert_eq!(
            Node::from(Product::new(vec![
                Tensor::new(vec![
                    Tensor::new(vec![
                        Number::new(3.0).into(),
                        Number::new(2.0).into(),
                        Number::new(1.0).into(),
                    ])
                    .into(),
                    Tensor::new(vec![
                        Number::new(1.0).into(),
                        Number::new(0.0).into(),
                        Number::new(2.0).into(),
                    ])
                    .into(),
                ])
                .into(),
                Tensor::new(vec![
                    Tensor::new(vec![
                        Number::new(1.0).into(),
                        Number::new(2.0).into(),
                    ])
                    .into(),
                    Tensor::new(vec![
                        Number::new(0.0).into(),
                        Number::new(1.0).into(),
                    ])
                    .into(),
                    Tensor::new(vec![
                        Number::new(4.0).into(),
                        Number::new(0.0).into(),
                    ])
                    .into(),
                ])
                .into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(7.0).into(),
                    Number::new(8.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(9.0).into(),
                    Number::new(2.0).into(),
                ])
                .into(),
            ])
            .into(),
        )
    }
}
