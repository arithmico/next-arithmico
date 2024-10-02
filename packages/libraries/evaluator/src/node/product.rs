use std::iter::zip;

use ast::{Node, Number, Product, Sum, Tensor};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Product {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::InvalidNode);
        }

        let mut elements = self
            .elements
            .iter()
            .map(|element| element.evaluate(context));

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
        (Node::Number(left), Node::Number(right)) => {
            multiply_numbers(left, right)
        }
        (Node::Number(left), Node::Tensor(right)) => {
            multiply_number_and_tensor(left, right, context)
        }
        (Node::Tensor(left), Node::Number(right)) => {
            multiply_number_and_tensor(right, left, context)
        }
        (Node::Tensor(left), Node::Tensor(right)) => {
            multiply_tensors(left, right, context)
        }
        _ => Err(EvaluateNodeError::UnsupportedOperation),
    }
}

fn multiply_numbers(
    left: &Number,
    right: &Number,
) -> Result<Node, EvaluateNodeError> {
    if !cfg!(feature = "operator_product_number_number") {
        return Err(EvaluateNodeError::UnsupportedOperation);
    }

    Ok(Number::new(left.value * right.value))
}

fn multiply_number_and_tensor(
    number: &Number,
    tensor: &Tensor,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    if !cfg!(feature = "operator_product_number_tensor") {
        return Err(EvaluateNodeError::UnsupportedOperation);
    }

    let elements = tensor
        .elements
        .iter()
        .map(|element| {
            Product::new(vec![Number::new(number.value), element.clone()])
                .evaluate(context)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Tensor::new_with_shape(tensor.shape.clone(), elements))
}

fn multiply_tensors(
    left: &Tensor,
    right: &Tensor,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    let left_rank = left.get_rank();
    let right_rank = right.get_rank();

    match (left_rank, right_rank) {
        (1, 1) => {
            if !cfg!(feature = "operator_product_number_number") {
                return Err(EvaluateNodeError::UnsupportedOperation);
            }

            if left.elements.len() != right.elements.len() {
                return Err(EvaluateNodeError::IncompatibleVectorDimensions(
                    left.elements.len(),
                    right.elements.len(),
                ));
            }

            let elements = zip(left.elements.iter(), right.elements.iter())
                .map(|(left, right)| {
                    Product::new(vec![left.clone(), right.clone()])
                });

            Sum::new(elements.collect()).evaluate(context)
        }
        _ => Err(EvaluateNodeError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use ast::Tensor;

    use super::*;

    #[test]
    fn evaluate_invalid_product() {
        let context = Context::default();
        let result = Product::new(vec![Number::new(1.)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::InvalidNode));
    }

    #[test]
    fn evaluate_product_number_number_2() {
        let context = Context::default();
        let result = Product::new(vec![Number::new(1.), Number::new(2.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(2.));
    }

    #[test]
    fn evaluate_product_number_number_3() {
        let context = Context::default();
        let result = Product::new(vec![Number::new(1.), Number::new(2.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(2.));
    }

    #[test]
    fn evaluate_product_vector_vector_2() {
        let context = Context::default();
        let result = Product::new(vec![
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ]),
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(14.));
    }

    #[test]
    fn evaluate_product_number_vector() {
        let context = Context::default();
        let result = Product::new(vec![
            Number::new(2.),
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new(2.),
                Number::new(4.),
                Number::new(6.),
            ]),
        );
    }

    #[test]
    fn evaluate_product_number_matrix() {
        let context = Context::default();
        let result = Product::new(vec![
            Number::new(2.),
            Tensor::new(vec![
                Tensor::new(vec![Number::new(1.), Number::new(2.)]),
                Tensor::new(vec![Number::new(3.), Number::new(4.)]),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![2, 2],
                vec![
                    Number::new(2.),
                    Number::new(4.),
                    Number::new(6.),
                    Number::new(8.),
                ]
            ),
        );
    }
}
