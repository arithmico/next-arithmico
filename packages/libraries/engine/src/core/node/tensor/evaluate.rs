use crate::core::{
    EvaluateNode, Context, EvaluateNodeError, GetNodeType, Node,
    Tensor,
};

impl EvaluateNode for Tensor {
    fn evaluate(
        &self,
        context: &Context,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_tensor") {
            return Err(EvaluateNodeError::unsupported_datatype(
                self.node_type(),
            ));
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
    use super::*;
    use crate::core::{Number, Sum};
    use trace::TracableMut;

    #[test]
    fn evaluate_empty_tensor() {
        let context = Context::default();
        let result = Tensor::new(vec![]).evaluate(&context).unwrap();
        assert_eq!(result, Tensor::new(vec![]));
    }

    #[test]
    fn evaluate_empty_tensor_with_trace() {
        let context = Context::default();
        let result = Tensor::new(vec![])
            .with_span(0, 1)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Tensor::new(vec![]).with_span(0, 1));
    }

    #[test]
    fn evaluate_tensor_with_numbers() {
        let context = Context::default();
        let result = Tensor::new(vec![Number::new(1.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Tensor::new(vec![Number::new(1.)]));
    }

    #[test]
    fn evaluate_tensor_with_sum() {
        let context = Context::default();
        let result =
            Tensor::new(vec![Sum::new(vec![Number::new(1.), Number::new(2.)])])
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Tensor::new(vec![Number::new(3.)]));
    }

    #[test]
    fn evaluate_tensor_with_sum_with_trace() {
        let context = Context::default();
        let result = Tensor::new(vec![Sum::new(vec![
            Number::new(1.).with_span(1, 1),
            Number::new(2.).with_span(3, 3),
        ])
        .with_span(1, 3)])
        .with_span(0, 4)
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![Number::new(3.).with_span(1, 3)]).with_span(0, 4)
        );
    }

    #[test]
    fn evaluate_tensor_preserve_shape() {
        let context = Context::default();
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
