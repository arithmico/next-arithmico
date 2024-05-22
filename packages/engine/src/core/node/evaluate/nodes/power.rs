use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, Node, Number},
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
            Node::Number(Number { value: left_value }),
            Node::Number(Number { value: right_value }),
        ) if cfg!(feature = "operator_power_number_number") => {
            Ok(Number::new(left_value.powf(right_value)).into())
        }
        _ => return Err(NodeEvaluationError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::Power;

    use super::*;

    #[test]
    fn evaluate_power() {
        let context = Context::default();
        assert_eq!(
            Node::from(Power::new(Number::new(6.0), Number::new(2.0),))
                .evaluate(&context)
                .unwrap(),
            Number::new(36.0).into()
        )
    }
}
