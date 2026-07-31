use node::{GetNodeType, Node, Number, Sum, Tensor};
use std::iter::zip;
use trace::{Tracable, TracableMut};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Sum {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::invalid_node(self.node_type()));
        }

        let mut elements = self
            .elements
            .iter()
            .map(|element| element.evaluate(context));

        let mut accumulator = elements.next().unwrap()?;
        for current_element in elements {
            accumulator =
                add_sum_elements(&accumulator, &current_element?, context)?;
        }

        Ok(accumulator)
    }
}

fn add_sum_elements(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Number(left), Node::Number(right)) => {
            if !cfg!(feature = "operator_sum_number_number") {
                return Err(EvaluateNodeError::unsupported_operation());
            }

            Ok(Number::new_node(left.value + right.value))
        }
        (Node::Tensor(left), Node::Tensor(right))
            if (left.get_rank() == 1
                && right.get_rank() == 1
                && cfg!(feature = "operator_sum_vector_vector"))
                || (left.get_rank() == 2
                    && right.get_rank() == 2
                    && cfg!(feature = "operator_sum_matrix_matrix"))
                || (left.get_rank() > 2
                    && right.get_rank() > 2
                    && cfg!(feature = "operator_sum_tensor_tensor")) =>
        {
            // TODO: add special error variants for vectors and matrices
            if left.shape != right.shape {
                return Err(EvaluateNodeError::incompatible_tensor_shapes(
                    &left.shape,
                    &right.shape,
                ));
            }

            Ok(Tensor::new_with_shape(
                left.shape.clone(),
                zip(left.elements.iter(), right.elements.iter())
                    .map(|(left, right)| {
                        Sum::new(vec![left.clone(), right.clone()])
                            .with_optional_span(left.hull())
                            .with_optional_span(right.hull())
                            .evaluate(context)
                    })
                    .collect::<Result<Vec<_>, EvaluateNodeError>>()?,
            ))
        }
        (left, right) => Err(EvaluateNodeError::unsupported_operation()
            .with_optional_span(left.hull())
            .with_optional_span(right.hull())),
    }
}

// TODO: add tests with traces
#[cfg(test)]
mod tests {
    use node::NodeType;

    use super::*;

    #[test]
    fn evaluate_invalid_sum() {
        let context = Context::default();
        let result = Sum::new(vec![Number::new_node(1.)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::invalid_node(NodeType::Sum)));
    }

    #[test]
    fn evaluate_sum_number_number_2() {
        let context = Context::default();
        let result = Sum::new(vec![Number::new_node(1.), Number::new_node(2.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(3.));
    }

    #[test]
    fn evaluate_sum_number_number_3() {
        let context = Context::default();
        let result = Sum::new(vec![
            Number::new_node(1.),
            Number::new_node(2.),
            Number::new_node(3.),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new_node(6.));
    }

    #[test]
    fn evaluate_sum_empty_vectors() {
        let context = Context::default();
        let result = Sum::new(vec![Tensor::new(vec![]), Tensor::new(vec![])])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Tensor::new(vec![]));
    }

    #[test]
    fn evaluate_vector_vector() {
        let context = Context::default();
        let result = Sum::new(vec![
            Tensor::new(vec![
                Number::new_node(1.0),
                Number::new_node(2.0),
                Number::new_node(3.0),
            ]),
            Tensor::new(vec![
                Number::new_node(1.0),
                Number::new_node(2.0),
                Number::new_node(3.0),
            ]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new_node(2.0),
                Number::new_node(4.0),
                Number::new_node(6.0),
            ])
        );
    }

    #[test]
    fn evaluate_vector_vector_err_incompatible_tensor_shapes() {
        let context = Context::default();
        let result = Sum::new(vec![
            Tensor::new(vec![
                Number::new_node(1.0),
                Number::new_node(2.0),
                Number::new_node(3.0),
            ]),
            Tensor::new(vec![Number::new_node(1.0), Number::new_node(2.0)]),
        ])
        .evaluate(&context)
        .unwrap_err();
        assert_eq!(
            result,
            EvaluateNodeError::incompatible_tensor_shapes(&[3], &[2])
        );
    }

    #[test]
    fn evaluate_matrix_matrix() {
        let context = Context::default();
        let result = Sum::new(vec![
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    // 2. row
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    // 3. row
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                ],
            ),
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    // 2. row
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    // 3. row
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                ],
            ),
        ])
        .evaluate(&context)
        .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new_node(2.0),
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    // 2. row
                    Number::new_node(0.0),
                    Number::new_node(2.0),
                    Number::new_node(0.0),
                    // 3. row
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    Number::new_node(2.0),
                ],
            ),
        );
    }

    #[test]
    fn evaluate_matrix_matrix_err_incompatible_tensor_shapes() {
        let context = Context::default();
        let result = Sum::new(vec![
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    // 2. row
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    // 3. row
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                ],
            ),
            Tensor::new_with_shape(
                vec![2, 2],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                ],
            ),
        ])
        .evaluate(&context)
        .unwrap_err();

        assert_eq!(
            result,
            EvaluateNodeError::incompatible_tensor_shapes(&[3, 3], &[2, 2])
        );
    }

    #[test]
    fn evaluate_tensor_tensor() {
        let context = Context::default();
        let result = Sum::new(vec![
            Tensor::new_with_shape(
                vec![2, 2, 2],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                ],
            ),
            Tensor::new_with_shape(
                vec![2, 2, 2],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                ],
            ),
        ])
        .evaluate(&context)
        .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![2, 2, 2],
                vec![
                    Number::new_node(2.0),
                    Number::new_node(0.0),
                    Number::new_node(2.0),
                    Number::new_node(0.0),
                    Number::new_node(2.0),
                    Number::new_node(0.0),
                    Number::new_node(2.0),
                    Number::new_node(0.0),
                ],
            ),
        );
    }

    #[test]
    fn evaluate_tensor_tensor_err_incompatible_tensor_shapes() {
        let context = Context::default();
        let result = Sum::new(vec![
            Tensor::new_with_shape(
                vec![2, 2, 2],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                ],
            ),
            Tensor::new_with_shape(
                vec![2, 2, 2, 2],
                vec![
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                    Number::new_node(1.0),
                    Number::new_node(0.0),
                ],
            ),
        ])
        .evaluate(&context)
        .unwrap_err();

        assert_eq!(
            result,
            EvaluateNodeError::incompatible_tensor_shapes(
                &[2, 2, 2],
                &[2, 2, 2, 2]
            )
        );
    }
}
