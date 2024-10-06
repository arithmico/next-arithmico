use ast::{Node, Tensor};
use common::{EvaluateNodeContext, EvaluateNodeError};

use crate::evaluate::EvaluateNode;

impl EvaluateNode for Tensor {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_tensor") {
            return Err(EvaluateNodeError::UnsupportedDataType(String::from(
                "tensor",
            )));
        }

        Ok(Tensor::new_with_shape(
            self.shape.clone(),
            self.elements
                .iter()
                .map(|element| element.evaluate(context))
                .collect::<Result<Vec<_>, EvaluateNodeError>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use ast::{Number, Sum};

    use super::*;

    #[test]
    fn evaluate_empty_tensor() {
        let context = EvaluateNodeContext::default();
        let result = Tensor::new(vec![]).evaluate(&context).unwrap();
        assert_eq!(result, Tensor::new(vec![]));
    }

    #[test]
    fn evaluate_tensor_with_numbers() {
        let context = EvaluateNodeContext::default();
        let result = Tensor::new(vec![Number::new(1.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Tensor::new(vec![Number::new(1.)]));
    }

    #[test]
    fn evaluate_tensor_with_sum() {
        let context = EvaluateNodeContext::default();
        let result =
            Tensor::new(vec![Sum::new(vec![Number::new(1.), Number::new(2.)])])
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Tensor::new(vec![Number::new(3.)]));
    }

    #[test]
    fn evaluate_tensor_preserve_shape() {
        let context = EvaluateNodeContext::default();
        let result = Tensor::new(vec![
            Tensor::new(vec![Number::new(1.), Number::new(2.)]),
            Tensor::new(vec![Number::new(3.), Number::new(4.)]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![2, 2],
                vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.),
                    Number::new(4.)
                ]
            )
        );
    }
}
