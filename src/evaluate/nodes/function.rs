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
