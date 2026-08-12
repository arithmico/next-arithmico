use evaluator::Error;
use node::{GetNodeType, IntoNode, Node, Tensor};

use crate::core::{Context, EvaluateNode};

impl EvaluateNode for Tensor {
    fn evaluate(&self, context: &Context) -> Result<Node, Error> {
        if !cfg!(feature = "datatype_tensor") {
            return Err(Error::unsupported_datatype(self.node_type()));
        }

        Ok(Tensor::new_with_shape(
            self.shape.clone(),
            self.elements
                .iter()
                .map(|element| element.evaluate(context))
                .collect::<Result<Vec<_>, Error>>()?,
        )
        .into_node())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Span;
    use node::{Number, Sum};
    use trace::TracableMut;

    #[test]
    fn evaluate_empty_tensor() {
        let context = Context::default();
        let result = Tensor::new_node(vec![]).evaluate(&context).unwrap();
        assert_eq!(result, Tensor::new_node(vec![]));
    }

    #[test]
    fn evaluate_empty_tensor_with_trace() {
        let context = Context::default();
        let result = Tensor::new_node(vec![])
            .with_span(Span::new_between(0, 1))
            .evaluate(&context)
            .unwrap();
        assert_eq!(
            result,
            Tensor::new_node(vec![]).with_span(Span::new_between(0, 1))
        );
    }

    #[test]
    fn evaluate_tensor_with_numbers() {
        let context = Context::default();
        let result = Tensor::new_node(vec![Number::new_node(1.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Tensor::new_node(vec![Number::new_node(1.)]));
    }

    #[test]
    fn evaluate_tensor_with_sum() {
        let context = Context::default();
        let result = Tensor::new_node(vec![Sum::new(vec![
            Number::new_node(1.),
            Number::new_node(2.),
        ])])
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Tensor::new_node(vec![Number::new_node(3.)]));
    }

    #[test]
    fn evaluate_tensor_with_sum_with_trace() {
        let context = Context::default();
        let result = Tensor::new_node(vec![
            Sum::new(vec![
                Number::new_node(1.).with_span(Span::new_between(1, 1)),
                Number::new_node(2.).with_span(Span::new_between(3, 3)),
            ])
            .with_span(Span::new_between(1, 3)),
        ])
        .with_span(Span::new_between(0, 4))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new_node(vec![
                Number::new_node(3.).with_span(Span::new_between(1, 3))
            ])
            .with_span(Span::new_between(0, 4))
        );
    }

    #[test]
    fn evaluate_tensor_preserve_shape() {
        let context = Context::default();
        let result = Tensor::new_node(vec![
            Tensor::new_node(vec![Number::new_node(1.), Number::new_node(2.)]),
            Tensor::new_node(vec![Number::new_node(3.), Number::new_node(4.)]),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![2, 2],
                vec![
                    Number::new_node(1.),
                    Number::new_node(2.),
                    Number::new_node(3.),
                    Number::new_node(4.)
                ]
            )
            .into_node()
        );
    }
}
