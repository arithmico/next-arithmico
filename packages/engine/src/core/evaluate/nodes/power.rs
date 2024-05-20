use crate::core::{
    context::Context, evaluate::NodeEvaluationError, node::Node,
};

pub fn evaluate_power(
    base: &Node,
    exponent: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_base = base.evaluate(context)?;
    let evaluated_exponent = exponent.evaluate(context)?;
    match (evaluated_base, evaluated_exponent) {
        (
            Node::Number { value: left_value },
            Node::Number { value: right_value },
        ) if cfg!(feature = "operator_power_number_number") => {
            Ok(Node::Number {
                value: left_value.powf(right_value),
            })
        }
        _ => return Err(NodeEvaluationError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_power() {
        let context = Context::default();
        assert_eq!(
            Node::Power {
                base: Box::new(Node::Number { value: 6.0 }),
                exponent: Box::new(Node::Number { value: 2.0 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 36.0 }
        )
    }
}
