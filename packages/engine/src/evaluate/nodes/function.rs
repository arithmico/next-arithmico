use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_function(
    arguments: &Vec<String>,
    expression: &Node,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(Node::Function {
        arguments: arguments.clone(),
        expression: Box::new(expression.clone()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_function() {
        let context = Context::default();
        assert_eq!(
            Node::Function {
                arguments: vec![String::from("x")],
                expression: Box::new(Node::Symbol {
                    name: String::from("x")
                })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Function {
                arguments: vec![String::from("x")],
                expression: Box::new(Node::Symbol {
                    name: String::from("x")
                })
            }
        )
    }
}
