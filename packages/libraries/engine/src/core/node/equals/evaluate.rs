use std::iter::zip;

use crate::core::{
    Boolean, Equals, EvaluateNode, EvaluateNodeContext, EvaluateNodeError, Node,
};

impl EvaluateNode for Equals {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                if !cfg!(feature = "operator_equals_number_number") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                Ok(Boolean::new(left.value == right.value))
            }
            (Node::Boolean(left), Node::Boolean(right)) => {
                if !cfg!(feature = "operator_equals_boolean_boolean") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                Ok(Boolean::new(left.value == right.value))
            }
            (Node::Tensor(left), Node::Tensor(right)) => {
                if !cfg!(feature = "operator_equals_tensor_tensor") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                if left.shape != right.shape {
                    return Ok(Boolean::new(false));
                }

                let comparison_result: Result<bool, EvaluateNodeError> =
                    zip(left.elements.into_iter(), right.elements.into_iter())
                        .map(|(left, right)| {
                            Equals::new(left, right).evaluate(context).map(
                                |element| {
                                    match element { Node::Boolean(element) => {
                            Ok(element.value)
                        } _ => {
                            Err(EvaluateNodeError::unsupported_operation())
                        }}
                                },
                            )
                        })
                        .try_fold(true, |acc, comparison_result| {
                            let is_equal = comparison_result??;
                            Ok(acc && is_equal)
                        });

                Ok(Boolean::new(comparison_result?))
            }
            _ => Err(EvaluateNodeError::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Number, Tensor};

    use super::*;

    #[test]
    fn evaluate_equals_boolean_boolean_true() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(Boolean::new(false), Boolean::new(false))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_equals_boolean_boolean_false() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(Boolean::new(false), Boolean::new(true))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_number_number_true() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(Number::new(3.), Number::new(3.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_equals_number_number_false() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(Number::new(3.), Number::new(4.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_true() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(
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
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ]),
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(4.),
            ]),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false_shape() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ]),
            Tensor::new(vec![
                Tensor::new(vec![Number::new(1.)]),
                Tensor::new(vec![Number::new(2.)]),
                Tensor::new(vec![Number::new(3.)]),
            ]),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false_incompatible_data_types() {
        let context = EvaluateNodeContext::default();
        let result = Equals::new(
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ]),
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Boolean::new(true),
            ]),
        )
        .evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::unsupported_operation()));
    }
}
