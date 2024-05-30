use crate::core::{context::Context, node::*};

impl EvaluateNode for Tensor {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeEvaluationError> {
        let mut evaluated_values = Vec::<Node>::new();
        for value in self.elements.iter() {
            let evaluated_value = value.evaluate(context)?;
            evaluated_values.push(evaluated_value);
        }
        Ok(Tensor::new_with_shape(evaluated_values, self.shape.clone())
            .unwrap()
            .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_vector() {
        let context = Context::default();
        assert_eq!(
            Node::from(Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Sum::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
                .into()
            ]))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
            ])
            .into()
        )
    }
}
