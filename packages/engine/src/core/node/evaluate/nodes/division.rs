use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, Division, Node, Number, Tensor},
};

pub fn evaluate_division(
    node: &Division,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_dividend = node.dividend.evaluate(context)?;
    let evaluated_divisor = node.divisor.evaluate(context)?;
    match (evaluated_dividend, evaluated_divisor) {
        (
            Node::Number(Number { value: left_value }),
            Node::Number(Number { value: right_value }),
        ) if cfg!(feature = "operator_division_number_number") => {
            if right_value == 0.0 {
                return Err(NodeEvaluationError::DivisionByZero);
            }
            Ok(Number::new(left_value / right_value).into())
        }
        (
            Node::Tensor(Tensor { elements: values }),
            Node::Number(Number { value }),
        ) if cfg!(feature = "operator_division_vector_number") => {
            if value == 0.0 {
                return Err(NodeEvaluationError::DivisionByZero);
            }
            Node::from(Tensor::new(
                values
                    .iter()
                    .map(|item| {
                        Node::from(Division::new(
                            item.clone(),
                            Number::new(value),
                        ))
                    })
                    .collect(),
            ))
            .evaluate(context)
        }
        _ => Err(NodeEvaluationError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_division_number_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Division::new(Number::new(6.0), Number::new(2.0)))
                .evaluate(&context)
                .unwrap(),
            Number::new(3.0).into()
        )
    }

    #[test]
    fn evaluate_division_number_zero() {
        let context = Context::default();
        assert_eq!(
            Node::from(Division::new(Number::new(6.0), Number::new(0.0)))
                .evaluate(&context)
                .err()
                .unwrap(),
            NodeEvaluationError::DivisionByZero
        )
    }

    #[test]
    fn evaluate_division_vector_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Division::new(
                Tensor::new(vec![
                    Number::new(2.0).into(),
                    Number::new(4.0).into(),
                    Number::new(6.0).into(),
                ]),
                Number::new(2.0)
            ))
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

    #[test]
    fn evaluate_division_vector_zero() {
        let context = Context::default();
        assert_eq!(
            Node::from(Division::new(
                Tensor::new(vec![
                    Number::new(2.0).into(),
                    Number::new(4.0).into(),
                    Number::new(6.0).into(),
                ]),
                Number::new(0.0)
            ))
            .evaluate(&context)
            .err()
            .unwrap(),
            NodeEvaluationError::DivisionByZero
        )
    }
}
