use std::iter::zip;

use crate::{
    EvaluateNode, EvaluateNodeContext, EvaluateNodeError, GetNodeType, Node,
    Number, Sum, Tensor,
};
use trace::TracableMut;

impl EvaluateNode for Sum {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
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
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Number(left), Node::Number(right)) => {
            if !cfg!(feature = "operator_sum_number_number") {
                return Err(EvaluateNodeError::unsupported_operation());
            }

            Ok(Number::new(left.value + right.value))
        }
        (Node::Tensor(left), Node::Tensor(right)) => {
            let left_rank = left.get_rank();
            let right_rank = right.get_rank();

            match (left_rank, right_rank) {
                (1, 1) => {
                    if !cfg!(feature = "operator_sum_vector_vector") {
                        return Err(EvaluateNodeError::unsupported_operation());
                    }

                    if left.elements.len() != right.elements.len() {
                        return Err(
                            EvaluateNodeError::incompatible_vector_dimensions(
                                left.elements.len(),
                                right.elements.len(),
                            ),
                        );
                    }

                    Ok(Tensor::new_with_shape(
                        left.shape.clone(),
                        zip(left.elements.iter(), right.elements.iter())
                            .map(|(left, right)| {
                                Sum::new(vec![left.clone(), right.clone()])
                                    .evaluate(context)
                            })
                            .collect::<Result<Vec<_>, EvaluateNodeError>>()?,
                    ))
                }
                _ => Err(EvaluateNodeError::unsupported_operation()),
            }
        }
        (_, right) => {
            Err(EvaluateNodeError::unsupported_operation().with_tracable(right))
        }
    }
    .map(|node| node.with_tracable((left, right)))
}

#[cfg(test)]
mod tests {
    use crate::NodeType;

    use super::*;

    #[test]
    fn evaluate_invalid_sum() {
        let context = EvaluateNodeContext::default();
        let result = Sum::new(vec![Number::new(1.)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::invalid_node(NodeType::Sum)));
    }

    #[test]
    fn evaluate_sum_number_number_2() {
        let context = EvaluateNodeContext::default();
        let result = Sum::new(vec![Number::new(1.), Number::new(2.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(3.));
    }

    #[test]
    fn evaluate_sum_number_number_3() {
        let context = EvaluateNodeContext::default();
        let result =
            Sum::new(vec![Number::new(1.), Number::new(2.), Number::new(3.)])
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Number::new(6.));
    }

    #[test]
    fn evaluate_sum_empty_vectors() {
        let context = EvaluateNodeContext::default();
        let result = Sum::new(vec![Tensor::new(vec![]), Tensor::new(vec![])])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Tensor::new(vec![]));
    }
}
