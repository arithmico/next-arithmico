use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_definition(
    symbol: &String,
    expression: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_expression = expression.evaluate(context)?;
    Ok(Node::Definition {
        symbol: symbol.clone(),
        expression: Box::new(evaluated_expression),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_definition() {
        let context = Context::default();
        assert_eq!(
            Node::Definition {
                symbol: String::from("test"),
                expression: Box::new(Node::Number { value: 42.0 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Definition {
                symbol: String::from("test"),
                expression: Box::new(Node::Number { value: 42.0 })
            }
        )
    }
}
