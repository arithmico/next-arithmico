use crate::core::{context::Context, node::*};

impl EvaluateNode for Negate {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        let evaluated_value = self.value.evaluate(context)?;
        match evaluated_value {
            Node::Number(Number { value })
                if cfg!(feature = "operator_negate_number") =>
            {
                Ok(Number::new(-value).into())
            }
            Node::Tensor(Tensor {
                elements: values, ..
            }) if cfg!(feature = "operator_negate_vector") => {
                let mut negated_values = Vec::<Node>::new();
                for value in values {
                    negated_values.push(
                        Node::from(Negate::new(value)).evaluate(context)?,
                    );
                }
                Ok(Tensor::new(negated_values).into())
            }
            Node::Boolean(node)
                if cfg!(feature = "operator_negate_boolean") =>
            {
                Ok(Boolean::new(!node.value).into())
            }
            _ => Err(NodeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn evaluate_negate_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Negate::new(Number::new(2.1)))
                .evaluate(&context)
                .unwrap(),
            Number::new(-2.1).into()
        )
    }

    #[test]
    fn evaluate_negate_vector() {
        let context = Context::default();
        assert_eq!(
            Node::from(Negate::new(Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into()
            ])))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Number::new(-1.0).into(),
                Number::new(-2.0).into(),
                Number::new(-3.0).into()
            ])
            .into()
        )
    }

    #[test]
    fn evaluate_negate_boolean_1() {
        let context = Context::default();
        assert_eq!(
            Node::from(Negate::new(Boolean::new(true)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(false).into()
        )
    }

    #[test]
    fn evaluate_negate_boolean_2() {
        let context = Context::default();
        assert_eq!(
            Node::from(Negate::new(Boolean::new(false)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(true).into()
        )
    }
}
