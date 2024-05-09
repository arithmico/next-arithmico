use crate::context::Context;
use crate::{evaluate::EvaluationError, node::Node};

pub fn evaluate_negate(
    value: &Node,
    context: &Context,
) -> Result<Node, EvaluationError> {
    let evaluated_value = value.evaluate(context)?;
    match evaluated_value {
        Node::Number { value } => Ok(Node::Number { value: -value }),
        _ => Err(EvaluationError::UnsupportedOperation),
    }
}
