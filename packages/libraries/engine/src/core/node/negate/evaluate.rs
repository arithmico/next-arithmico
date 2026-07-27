use node::{Boolean, Negate, Node, Number, Tensor};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Negate {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        let value = self.value.evaluate(context)?;

        match value {
            Node::Number(number) => {
                if !cfg!(feature = "operator_negate_number") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                Ok(Number::new_node(-number.value))
            }
            Node::Boolean(boolean) => {
                if !cfg!(feature = "operator_negate_boolean") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                Ok(Boolean::new(!boolean.value))
            }
            Node::Tensor(tensor) => {
                if !cfg!(feature = "operator_negate_tensor") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                let elements = tensor
                    .elements
                    .iter()
                    .map(|element| {
                        Negate::new(element.clone()).evaluate(context)
                    })
                    .collect::<Result<Vec<_>, EvaluateNodeError>>()?;

                Ok(Tensor::new_with_shape(tensor.shape.clone(), elements))
            }
            _ => Err(EvaluateNodeError::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn evaluate_negate_number() {
        let context = Context::default();
        let result = Negate::new(Number::new_node(42.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(-42.));
    }

    #[test]
    fn evaluate_negate_number_with_trace() {
        let context = Context::default();
        let result = Negate::new(Number::new_node(42.).with_span(1, 2))
            .with_span(0, 2)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(-42.).with_span(0, 2));
    }

    #[test]
    fn evaluate_negate_boolean() {
        let context = Context::default();
        let result =
            Negate::new(Boolean::new(true)).evaluate(&context).unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_negate_boolean_with_trace() {
        let context = Context::default();
        let result = Negate::new(Boolean::new(true).with_span(1, 1))
            .with_span(0, 1)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false).with_span(0, 1));
    }

    #[test]
    fn evaluate_negate_tensor() {
        let context = Context::default();
        let result = Negate::new(Tensor::new(vec![
            Number::new_node(1.),
            Number::new_node(2.),
            Number::new_node(3.),
        ]))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new_node(-1.),
                Number::new_node(-2.),
                Number::new_node(-3.),
            ])
        );
    }

    #[test]
    fn evaluate_negate_tensor_with_trace() {
        let context = Context::default();
        let result = Negate::new(
            Tensor::new(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ])
            .with_span(1, 5),
        )
        .with_span(0, 5)
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new_node(-1.),
                Number::new_node(-2.),
                Number::new_node(-3.),
            ])
            .with_span(0, 5)
        );
    }
}
