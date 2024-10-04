use ast::{Boolean, Negate, Node, Number, Tensor};

use crate::{evaluate::EvaluateNode, EvaluateNodeContext, EvaluateNodeError};

impl EvaluateNode for Negate {
    fn evaluate(&self, context: &EvaluateNodeContext) -> Result<Node, EvaluateNodeError> {
        let value = self.value.evaluate(context)?;

        match value {
            Node::Number(number) => {
                if !cfg!(feature = "operator_negate_number") {
                    return Err(EvaluateNodeError::UnsupportedOperation);
                }

                Ok(Number::new(-number.value))
            }
            Node::Boolean(boolean) => {
                if !cfg!(feature = "operator_negate_boolean") {
                    return Err(EvaluateNodeError::UnsupportedOperation);
                }

                Ok(Boolean::new(!boolean.value))
            }
            Node::Tensor(tensor) => {
                if !cfg!(feature = "operator_negate_tensor") {
                    return Err(EvaluateNodeError::UnsupportedOperation);
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
            _ => Err(EvaluateNodeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negate_number() {
        let context = EvaluateNodeContext::default();
        let result = Negate::new(Number::new(42.)).evaluate(&context).unwrap();
        assert_eq!(result, Number::new(-42.));
    }

    #[test]
    fn evaluate_negate_boolean() {
        let context = EvaluateNodeContext::default();
        let result =
            Negate::new(Boolean::new(true)).evaluate(&context).unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_negate_tensor() {
        let context = EvaluateNodeContext::default();
        let result = Negate::new(Tensor::new(vec![
            Number::new(1.),
            Number::new(2.),
            Number::new(3.),
        ]))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new(-1.),
                Number::new(-2.),
                Number::new(-3.),
            ])
        );
    }
}
