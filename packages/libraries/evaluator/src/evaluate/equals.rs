use std::iter::zip;

use node::{Boolean, Equals, Node};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Equals {
    fn evaluate(&self, context: Options) -> Result<Node, Error> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                if !cfg!(feature = "operator_equals_number_number") {
                    return Err(Error::unsupported_operation());
                }

                Ok(Boolean::new(left.value == right.value))
            }
            (Node::Boolean(left), Node::Boolean(right)) => {
                if !cfg!(feature = "operator_equals_boolean_boolean") {
                    return Err(Error::unsupported_operation());
                }

                Ok(Boolean::new(left.value == right.value))
            }
            (Node::Tensor(left), Node::Tensor(right)) => {
                if !cfg!(feature = "operator_equals_tensor_tensor") {
                    return Err(Error::unsupported_operation());
                }

                if left.shape != right.shape {
                    return Ok(Boolean::new(false));
                }

                let comparison_result: Result<bool, Error> =
                    zip(left.elements.into_iter(), right.elements.into_iter())
                        .map(|(left, right)| {
                            Equals::new(left, right).evaluate(context).map(
                                |element| match element {
                                    Node::Boolean(element) => Ok(element.value),
                                    _ => Err(Error::unsupported_operation()),
                                },
                            )
                        })
                        .try_fold(true, |acc, comparison_result| {
                            let is_equal = comparison_result??;
                            Ok(acc && is_equal)
                        });

                Ok(Boolean::new(comparison_result?))
            }
            _ => Err(Error::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use node::{Number, Tensor};

    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_equals_boolean_boolean_true() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(Boolean::new(false), Boolean::new(false))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_equals_boolean_boolean_false() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(Boolean::new(false), Boolean::new(true))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_number_number_true() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(Number::new_node(3.), Number::new_node(3.))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_equals_number_number_false() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(Number::new_node(3.), Number::new_node(4.))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_true() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(4.),
            ]),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false_shape() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
            Tensor::new_node(vec![
                Tensor::new_node(vec![Number::new_node(1.)]),
                Tensor::new_node(vec![Number::new_node(2.)]),
                Tensor::new_node(vec![Number::new_node(3.)]),
            ]),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false_incompatible_data_types() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Equals::new(
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ]),
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Boolean::new(true),
            ]),
        )
        .evaluate(options);
        assert_eq!(result, Err(Error::unsupported_operation()));
    }
}
