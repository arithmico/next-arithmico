use crate::context::Context;
use crate::{evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_division(
    dividend: &Node,
    divisor: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_dividend = dividend.evaluate(context)?;
    let evaluated_divisor = divisor.evaluate(context)?;
    match (evaluated_dividend, evaluated_divisor) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) if cfg!(feature = "operator_division_number_number") => {
            if right_value == 0.0 {
                return Err(NodeEvaluationError::DivisionByZero);
            }
            Ok(Node::Number {
                value: left_value / right_value,
            })
        }
        (Node::Vector { values }, Node::Number { value })
            if cfg!(feature = "operator_division_vector_number") =>
        {
            if value == 0.0 {
                return Err(NodeEvaluationError::DivisionByZero);
            }
            Node::Vector {
                values: values
                    .iter()
                    .map(|item| Node::Division {
                        dividend: item.clone().into(),
                        divisor: Node::Number { value }.into(),
                    })
                    .collect(),
            }
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
            Node::Division {
                dividend: Box::new(Node::Number { value: 6.0 }),
                divisor: Box::new(Node::Number { value: 2.0 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 3.0 }
        )
    }

    #[test]
    fn evaluate_division_number_zero() {
        let context = Context::default();
        assert_eq!(
            Node::Division {
                dividend: Box::new(Node::Number { value: 6.0 }),
                divisor: Box::new(Node::Number { value: 0.0 })
            }
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
            Node::Division {
                dividend: Box::new(Node::Vector {
                    values: vec![
                        Node::Number { value: 2.0 },
                        Node::Number { value: 4.0 },
                        Node::Number { value: 6.0 },
                    ]
                }),
                divisor: Box::new(Node::Number { value: 2.0 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            }
        )
    }

    #[test]
    fn evaluate_division_vector_zero() {
        let context = Context::default();
        assert_eq!(
            Node::Division {
                dividend: Box::new(Node::Vector {
                    values: vec![
                        Node::Number { value: 2.0 },
                        Node::Number { value: 4.0 },
                        Node::Number { value: 6.0 },
                    ]
                }),
                divisor: Box::new(Node::Number { value: 0.0 })
            }
            .evaluate(&context)
            .err()
            .unwrap(),
            NodeEvaluationError::DivisionByZero
        )
    }
}
