use crate::core::{
    context::Context, evaluate::NodeEvaluationError, node::Node,
};

pub fn evaluate_negate(
    value: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_value = value.evaluate(context)?;
    match evaluated_value {
        Node::Number { value } if cfg!(feature = "operator_negate_number") => {
            Ok(Node::Number { value: -value })
        }
        Node::Vector { values } if cfg!(feature = "operator_negate_vector") => {
            let mut negated_values = Vec::<Node>::new();
            for value in values {
                negated_values.push(evaluate_negate(&value, context)?);
            }
            Ok(Node::Vector {
                values: negated_values,
            })
        }
        _ => Err(NodeEvaluationError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negate_number() {
        let context = Context::default();
        assert_eq!(
            Node::Negate {
                value: Box::new(Node::Number { value: 2.1 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: -2.1 }
        )
    }

    #[test]
    fn evaluate_negate_vector() {
        let context = Context::default();
        assert_eq!(
            Node::Negate {
                value: Box::new(Node::Vector {
                    values: vec![
                        Node::Number { value: 1.0 },
                        Node::Number { value: 2.0 },
                        Node::Number { value: 3.0 },
                    ]
                })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: -1.0 },
                    Node::Number { value: -2.0 },
                    Node::Number { value: -3.0 },
                ]
            }
        )
    }
}
