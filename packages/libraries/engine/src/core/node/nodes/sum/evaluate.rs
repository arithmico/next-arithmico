use std::iter::zip;

use crate::core::{context::Context, node::*};

impl EvaluateNode for Sum {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        if self.values.len() < 2 {
            return Err(NodeError::RuntimeError(
                "invalid number of operands".into(),
            ));
        }
        let mut result = self.values[0].evaluate(context)?;
        for current_node in self.values[1..].iter() {
            let evaluated_current_node = current_node.evaluate(context)?;
            result = add_nodes(&result, &evaluated_current_node, context)?;
        }

        Ok(result)
    }
}

fn add_nodes(
    left: &Node,
    right: &Node,
    context: &Context,
) -> Result<Node, NodeError> {
    match (left, right) {
        (
            Node::Number(Number { value: left_value }),
            Node::Number(Number { value: right_value }),
        ) if cfg!(feature = "operator_sum_number_number") => {
            Ok(Number::new(left_value + right_value).into())
        }
        (Node::Tensor(left), Node::Tensor(right))
            if cfg!(feature = "operator_sum_vector_vector") =>
        {
            if left.shape != right.shape {
                return Err(NodeError::RuntimeError(
                    "unable to perform vector addition due to mismatching dimensions".into(),
                ));
            }
            Node::from(Tensor::new(
                zip(&left.elements, &right.elements)
                    .map(|(left, right)| {
                        Node::from(Sum::new(vec![left.clone(), right.clone()]))
                    })
                    .collect(),
            ))
            .evaluate(context)
        }
        _ => return Err(NodeError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_sum_number_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Sum::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Number::new(3.0).into()
        )
    }

    #[test]
    fn evaluate_sum_large_number_large_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Sum::new(vec![
                Power::new(Number::new(10.0), Number::new(32.0)).into(),
                Negate::new(Power::new(Number::new(10.0), Number::new(32.0)))
                    .into()
            ]))
            .evaluate(&context)
            .unwrap(),
            Number::new(0.0).into()
        )
    }

    #[test]
    fn evaluate_sum_vector_vector() {
        let context = Context::default();
        assert_eq!(
            Node::from(Sum::new(vec![
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(3.0).into(),
                    Number::new(2.0).into(),
                    Number::new(1.0).into(),
                ])
                .into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Number::new(4.0).into(),
                Number::new(4.0).into(),
                Number::new(4.0).into(),
            ])
            .into(),
        )
    }
}
