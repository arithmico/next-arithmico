use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_function(
    arguments: &Vec<String>,
    expression: &Node,
    _context: &Context,
) -> Result<Node, EvaluationError> {
    Ok(Node::Function {
        arguments: arguments.clone(),
        expression: Box::new(expression.clone()),
    })
}
