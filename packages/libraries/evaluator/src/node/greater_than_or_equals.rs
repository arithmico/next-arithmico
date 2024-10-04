use ast::{Boolean, GreaterThanOrEquals, Node};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for GreaterThanOrEquals {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                Ok(Boolean::new(left.value >= right.value))
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
    fn evaluate_greater_than_or_equals_number_number_true() {
        let context = Context::default();
        let result = GreaterThanOrEquals::new(Number::new(2.), Number::new(1.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_greater_than_or_equals_number_number_true_equals() {
        let context = Context::default();
        let result = GreaterThanOrEquals::new(Number::new(2.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_greater_than_or_equals_number_number_false() {
        let context = Context::default();
        let result = GreaterThanOrEquals::new(Number::new(1.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }
}
