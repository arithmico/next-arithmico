use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_definition(
    symbol: &String,
    expression: &Node,
    context: &Context,
) -> Result<Node, EvaluationError> {
    let evaluated_expression = expression.evaluate(context)?;
    Ok(Node::Definition {
        symbol: symbol.clone(),
        expression: Box::new(evaluated_expression),
    })
}
