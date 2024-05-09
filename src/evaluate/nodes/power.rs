use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_power(
    base: &Node,
    exponent: &Node,
    context: &Context,
) -> Result<Node, EvaluationError> {
    let evaluated_base = base.evaluate(context)?;
    let evaluated_exponent = exponent.evaluate(context)?;
    match (evaluated_base, evaluated_exponent) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) => Ok(Node::Number {
            value: left_value.powf(right_value),
        }),
        _ => return Err(EvaluationError::UnsupportedOperation),
    }
}
