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
        ) => Ok(Node::Number {
            value: left_value / right_value,
        }),
        _ => return Err(NodeEvaluationError::UnsupportedOperation),
    }
}
