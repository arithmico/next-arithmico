use crate::context::Context;
use crate::node::Node;

use super::nodes::{
    evaluate_division, evaluate_negate, evaluate_number, evaluate_power,
    evaluate_product, evaluate_sum, evaluate_symbol,
};
use super::EvaluationError;

impl Node {
    pub fn evaluate(&self, context: &Context) -> Result<Node, EvaluationError> {
        match self {
            Node::Number { value } => evaluate_number(value, context),
            Node::Negate { value } => evaluate_negate(value, context),
            Node::Sum { values } => evaluate_sum(values, context),
            Node::Product { values } => evaluate_product(values, context),
            Node::Division { dividend, divisor } => {
                evaluate_division(dividend, divisor, context)
            }
            Node::Power { base, exponent } => {
                evaluate_power(base, exponent, context)
            }
            Node::Symbol { name } => evaluate_symbol(name, context),
            _ => Err(EvaluationError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        assert_eq!(
            Node::Number { value: 2.1 }
                .evaluate(&Context::new())
                .unwrap(),
            Node::Number { value: 2.1 }
        )
    }

    #[test]
    fn evaluate_negate_number() {
        assert_eq!(
            Node::Negate {
                value: Box::new(Node::Number { value: 2.1 })
            }
            .evaluate(&Context::new())
            .unwrap(),
            Node::Number { value: -2.1 }
        )
    }

    #[test]
    fn evaluate_sum() {
        assert_eq!(
            Node::Sum {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 }
                ]
            }
            .evaluate(&Context::new())
            .unwrap(),
            Node::Number { value: 3.0 }
        )
    }

    #[test]
    fn evaluate_product() {
        assert_eq!(
            Node::Product {
                values: vec![
                    Node::Number { value: 3.0 },
                    Node::Number { value: 2.0 }
                ]
            }
            .evaluate(&Context::new())
            .unwrap(),
            Node::Number { value: 6.0 }
        )
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(
            Node::Division {
                dividend: Box::new(Node::Number { value: 6.0 }),
                divisor: Box::new(Node::Number { value: 2.0 })
            }
            .evaluate(&Context::new())
            .unwrap(),
            Node::Number { value: 3.0 }
        )
    }

    #[test]
    fn evaluate_power() {
        assert_eq!(
            Node::Power {
                base: Box::new(Node::Number { value: 6.0 }),
                exponent: Box::new(Node::Number { value: 2.0 })
            }
            .evaluate(&Context::new())
            .unwrap(),
            Node::Number { value: 36.0 }
        )
    }

    #[test]
    fn evaluate_symbol() {
        let mut context = Context::new();
        context.insert("test", Node::Number { value: 36.0 });

        assert_eq!(
            Node::Symbol {
                name: String::from("test")
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 36.0 }
        )
    }
}
