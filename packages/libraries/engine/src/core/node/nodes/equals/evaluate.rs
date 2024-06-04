use std::iter::zip;

use crate::core::{
    context::Context,
    node::{And, Boolean, EvaluateNode, Node, NodeError, Number, Tensor},
};

use super::Equals;

impl EvaluateNode for Equals {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right))
                if cfg!(feature = "operator_equals_number_number") =>
            {
                compare_numbers(left, right)
            }
            (Node::Boolean(left), Node::Boolean(right))
                if cfg!(feature = "operator_equals_boolean_boolean") =>
            {
                compare_booleans(left, right)
            }
            (Node::Tensor(left), Node::Tensor(right))
                if cfg!(feature = "operator_equals_tensor_tensor") =>
            {
                compare_tensors(left, right, context)
            }
            _ => Err(NodeError::UnsupportedOperation),
        }
    }
}

fn compare_numbers(left: Number, right: Number) -> Result<Node, NodeError> {
    Ok(Boolean::new(left.value == right.value).into())
}

fn compare_booleans(left: Boolean, right: Boolean) -> Result<Node, NodeError> {
    Ok(Boolean::new(left.value == right.value).into())
}

fn compare_tensors(
    left: Tensor,
    right: Tensor,
    context: &Context,
) -> Result<Node, NodeError> {
    if left.shape != right.shape {
        return Ok(Boolean::new(false).into());
    }

    And::new(
        zip(left.elements, right.elements)
            .map(|(left, right)| Node::from(Equals::new(left, right)))
            .collect(),
    )
    .evaluate(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_equals_number_number_true() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(Number::new(6.0), Number::new(6.0)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(true).into()
        )
    }

    #[test]
    fn evaluate_equals_number_number_false() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(Number::new(6.0), Number::new(5.0)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(false).into()
        )
    }

    #[test]
    fn evaluate_equals_boolean_boolean_true_1() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(Boolean::new(true), Boolean::new(true)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(true).into()
        )
    }

    #[test]
    fn evaluate_equals_boolean_boolean_true_2() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(Boolean::new(false), Boolean::new(false)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(true).into()
        )
    }

    #[test]
    fn evaluate_equals_boolean_boolean_false_1() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(Boolean::new(false), Boolean::new(true)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(false).into()
        )
    }

    #[test]
    fn evaluate_equals_boolean_boolean_false_2() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(Boolean::new(true), Boolean::new(false)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(false).into()
        )
    }

    #[test]
    fn evaluate_equals_tensor_tensor_true() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ]),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
            ))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(true).into()
        )
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false_different_shape() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ]),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
            ))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(false).into()
        )
    }

    #[test]
    fn evaluate_equals_tensor_tensor_false_different_element() {
        let context = Context::default();
        assert_eq!(
            Node::from(Equals::new(
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ]),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(3.0).into(),
                ])
            ))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(false).into()
        )
    }
}
