use evaluator::Error;
use node::{Boolean, LessThanOrEquals, Node};

use crate::core::{Context, EvaluateNode};

impl EvaluateNode for LessThanOrEquals {
    fn evaluate(&self, context: &Context) -> Result<Node, Error> {
        let left = self.left.evaluate(context)?;
        let right = self.right.evaluate(context)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                Ok(Boolean::new(left.value <= right.value))
            }
            _ => Err(Error::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use node::Number;

    use super::*;

    #[test]
    fn evaluate_less_than_or_equals_number_number_true() {
        let context = Context::default();
        let result =
            LessThanOrEquals::new(Number::new_node(1.), Number::new_node(2.))
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_less_than_or_equals_number_number_true_equals() {
        let context = Context::default();
        let result =
            LessThanOrEquals::new(Number::new_node(2.), Number::new_node(2.))
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_less_than_or_equals_number_number_false() {
        let context = Context::default();
        let result =
            LessThanOrEquals::new(Number::new_node(2.), Number::new_node(1.))
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Boolean::new(false));
    }
}
