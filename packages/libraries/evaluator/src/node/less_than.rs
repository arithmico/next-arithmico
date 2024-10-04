use ast::{Boolean, LessThan, Node};

use crate::{evaluate::EvaluateNode, EvaluateNodeContext, EvaluateNodeError};

impl EvaluateNode for LessThan {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                Ok(Boolean::new(left.value < right.value))
            }
            _ => Err(EvaluateNodeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::Number;

    use super::*;

    #[test]
    fn evaluate_less_than_number_number_true() {
        let context = EvaluateNodeContext::default();
        let result = LessThan::new(Number::new(1.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_less_than_number_number_false() {
        let context = EvaluateNodeContext::default();
        let result = LessThan::new(Number::new(2.), Number::new(1.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }
}
