use crate::core::{
    context::Context,
    node::{Boolean, EvaluateNode, Node, NodeError, Number},
};

use super::GreaterThan;

impl EvaluateNode for GreaterThan {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right))
                if cfg!(feature = "operator_greater_than_number_number") =>
            {
                compare_numbers(left, right)
            }
            _ => Err(NodeError::UnsupportedOperation),
        }
    }
}

fn compare_numbers(left: Number, right: Number) -> Result<Node, NodeError> {
    Ok(Boolean::new(left.value > right.value).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_greater_than_number_number_false_equals() {
        let context = Context::default();
        assert_eq!(
            Node::from(GreaterThan::new(Number::new(6.0), Number::new(6.0)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(false).into()
        )
    }

    #[test]
    fn evaluate_greater_than_number_number_true() {
        let context = Context::default();
        assert_eq!(
            Node::from(GreaterThan::new(Number::new(7.0), Number::new(6.0)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(true).into()
        )
    }

    #[test]
    fn evaluate_greater_than_number_number_false_less() {
        let context = Context::default();
        assert_eq!(
            Node::from(GreaterThan::new(Number::new(4.0), Number::new(5.0)))
                .evaluate(&context)
                .unwrap(),
            Boolean::new(false).into()
        )
    }
}
