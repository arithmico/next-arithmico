use node::{Boolean, GreaterThanOrEquals, Node};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for GreaterThanOrEquals {
    fn evaluate(&self, options: Options) -> Result<Node, Error> {
        let left = self.left.evaluate(options)?;
        let right = self.right.evaluate(options)?;

        match (left, right) {
            (Node::Number(left), Node::Number(right)) => {
                Ok(Boolean::new(left.value >= right.value))
            }
            _ => Err(Error::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use node::Number;

    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_greater_than_or_equals_number_number_true() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = GreaterThanOrEquals::new(
            Number::new_node(2.),
            Number::new_node(1.),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_greater_than_or_equals_number_number_true_equals() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = GreaterThanOrEquals::new(
            Number::new_node(2.),
            Number::new_node(2.),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_greater_than_or_equals_number_number_false() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = GreaterThanOrEquals::new(
            Number::new_node(1.),
            Number::new_node(2.),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }
}
