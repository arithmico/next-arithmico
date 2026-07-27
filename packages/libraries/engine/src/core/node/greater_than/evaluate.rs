use node::{Boolean, GreaterThan, Node};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for GreaterThan {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                Ok(Boolean::new(left.value > right.value))
            }
            _ => Err(EvaluateNodeError::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use node::Number;

    use super::*;

    #[test]
    fn evaluate_greater_than_number_number_true() {
        let context = Context::default();
        let result =
            GreaterThan::new(Number::new_node(2.), Number::new_node(1.))
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_greater_than_number_number_false() {
        let context = Context::default();
        let result =
            GreaterThan::new(Number::new_node(1.), Number::new_node(2.))
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Boolean::new(false));
    }
}
