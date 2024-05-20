use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_vector(
    values: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let mut evaluated_values = Vec::<Node>::new();
    for value in values {
        let evaluated_value = value.evaluate(context)?;
        evaluated_values.push(evaluated_value);
    }
    Ok(Node::Vector {
        values: evaluated_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_vector() {
        let context = Context::default();
        assert_eq!(
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Sum {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                        ]
                    }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            },
        )
    }
}
