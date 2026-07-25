use std::iter::zip;

use node::{
    GetNodeType, Node, Number, Product, Sum, Tensor, convert_to_outer_index,
};
use trace::{Tracable, TracableMut};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Product {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::invalid_node(self.node_type()));
        }

        let mut elements = self
            .elements
            .iter()
            .map(|element| element.evaluate(context));

        // Safety: this can not panic due to the previous length check
        let mut accumulator = elements.next().unwrap()?;
        for current_element in elements {
            accumulator = multiply_product_elements(
                &accumulator,
                &current_element?,
                context,
            )?;
        }

        Ok(accumulator)
    }
}

fn multiply_product_elements(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Number(left), Node::Number(right))
            if cfg!(feature = "operator_product_number_number") =>
        {
            multiply_numbers(left, right)
        }
        (Node::Number(left), Node::Tensor(right))
            if cfg!(feature = "operator_product_number_tensor") =>
        {
            multiply_number_and_tensor(left, right, context)
        }
        (Node::Tensor(left), Node::Number(right))
            if cfg!(feature = "operator_product_number_tensor") =>
        {
            multiply_number_and_tensor(right, left, context)
        }
        (Node::Tensor(left), Node::Tensor(right))
            if left.get_rank() == 1
                && right.get_rank() == 1
                && cfg!(feature = "operator_product_vector_vector") =>
        {
            multiply_vectors(left, right, context)
        }
        (Node::Tensor(left), Node::Tensor(right))
            if left.get_rank() == 2
                && right.get_rank() == 2
                && cfg!(feature = "operator_product_matrix_matrix") =>
        {
            multiply_matrices(left, right, context)
        }
        (left, right) => Err(EvaluateNodeError::unsupported_operation()
            .with_optional_span(left.hull())
            .with_optional_span(right.hull())),
    }
}

fn multiply_numbers(
    left: &Number,
    right: &Number,
) -> Result<Node, EvaluateNodeError> {
    Ok(Number::new_node(left.value * right.value))
}

fn multiply_number_and_tensor(
    number: &Number,
    tensor: &Tensor,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    let elements = tensor
        .elements
        .iter()
        .map(|element| {
            Product::new(vec![Number::new_node(number.value), element.clone()])
                .evaluate(context)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Tensor::new_with_shape(tensor.shape.clone(), elements))
}

fn multiply_vectors(
    left: &Tensor,
    right: &Tensor,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    debug_assert_eq!(left.get_rank(), 1);
    debug_assert_eq!(right.get_rank(), 1);

    if left.elements.len() != right.elements.len() {
        return Err(EvaluateNodeError::incompatible_vector_dimensions(
            left.elements.len(),
            right.elements.len(),
        ));
    }

    let elements = zip(left.elements.iter(), right.elements.iter())
        .map(|(left, right)| Product::new(vec![left.clone(), right.clone()]));

    Sum::new(elements.collect()).evaluate(context)
}

fn multiply_matrices(
    left: &Tensor,
    right: &Tensor,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    debug_assert_eq!(left.get_rank(), 2);
    debug_assert_eq!(right.get_rank(), 2);

    if left.shape.get(1).unwrap() != right.shape.get(0).unwrap() {
        return Err(EvaluateNodeError::incompatible_matrix_dimensions(
            left.shape.clone(),
            right.shape.clone(),
        ));
    }

    let x = *left.shape.get(1).unwrap();
    let dim0 = *left.shape.get(0).unwrap();
    let dim1 = *right.shape.get(1).unwrap();
    let result_shape = vec![dim0, dim1];

    let elements = (0usize..(dim0 * dim1))
        .map(|index| {
            let outer_index =
                convert_to_outer_index(&result_shape, index).unwrap();
            let i = *outer_index.get(0).unwrap();
            let k = *outer_index.get(1).unwrap();

            Sum::new(
                (0usize..x)
                    .map(|j| {
                        Product::new(vec![
                            left.get_element(&vec![i, j]).unwrap().clone(),
                            right.get_element(&vec![j, k]).unwrap().clone(),
                        ])
                    })
                    .collect(),
            )
        })
        .collect();

    Tensor::new_with_shape(result_shape, elements).evaluate(context)
}

// TODO: tests with traces
#[cfg(test)]
mod tests {
    use node::{NodeType, Tensor};

    use super::*;

    #[test]
    fn evaluate_invalid_product() {
        let context = Context::default();
        let result =
            Product::new(vec![Number::new_node(1.)]).evaluate(&context);
        assert_eq!(
            result,
            Err(EvaluateNodeError::invalid_node(NodeType::Product))
        );
    }

    #[test]
    fn evaluate_product_number_number_2() {
        let context = Context::default();
        let result =
            Product::new(vec![Number::new_node(1.), Number::new_node(2.)])
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Number::new_node(2.));
    }

    #[test]
    fn evaluate_product_number_number_3() {
        let context = Context::default();
        let result =
            Product::new(vec![Number::new_node(1.), Number::new_node(2.)])
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Number::new_node(2.));
    }

    #[test]
    fn evaluate_product_vector_vector_2() {
        let context = Context::default();
        let result = Product::new(vec![
            Tensor::new(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
            Tensor::new(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new_node(14.));
    }

    #[test]
    fn evaluate_product_number_vector() {
        let context = Context::default();
        let result = Product::new(vec![
            Number::new_node(2.),
            Tensor::new(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new_node(2.),
                Number::new_node(4.),
                Number::new_node(6.),
            ]),
        );
    }

    #[test]
    fn evaluate_product_number_matrix() {
        let context = Context::default();
        let result = Product::new(vec![
            Number::new_node(2.),
            Tensor::new(vec![
                Tensor::new(vec![Number::new_node(1.), Number::new_node(2.)]),
                Tensor::new(vec![Number::new_node(3.), Number::new_node(4.)]),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![2, 2],
                vec![
                    Number::new_node(2.),
                    Number::new_node(4.),
                    Number::new_node(6.),
                    Number::new_node(8.),
                ]
            ),
        );
    }

    #[test]
    fn evaluate_product_matrix_matrix() {
        let context = Context::default();
        assert_eq!(
            Product::new(vec![
                Tensor::new(vec![
                    Tensor::new(vec![
                        Number::new_node(3.0),
                        Number::new_node(2.0),
                        Number::new_node(1.0),
                    ]),
                    Tensor::new(vec![
                        Number::new_node(1.0),
                        Number::new_node(0.0),
                        Number::new_node(2.0),
                    ]),
                ]),
                Tensor::new(vec![
                    Tensor::new(vec![
                        Number::new_node(1.0),
                        Number::new_node(2.0),
                    ]),
                    Tensor::new(vec![
                        Number::new_node(0.0),
                        Number::new_node(1.0),
                    ]),
                    Tensor::new(vec![
                        Number::new_node(4.0),
                        Number::new_node(0.0),
                    ]),
                ]),
            ])
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Tensor::new(
                    vec![Number::new_node(7.0), Number::new_node(8.0),]
                ),
                Tensor::new(
                    vec![Number::new_node(9.0), Number::new_node(2.0),]
                ),
            ]),
        )
    }
}
