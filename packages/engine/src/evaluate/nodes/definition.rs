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
