use crate::node::Node;

use super::nodes::{
    evaluate_division, evaluate_negate, evaluate_number, evaluate_power,
    evaluate_product, evaluate_sum,
};
use super::EvaluationError;

impl Node {
    pub fn evaluate(&self) -> Result<Node, EvaluationError> {
        match self {
            Node::Number { value } => evaluate_number(value),
            Node::Negate { value } => evaluate_negate(value),
            Node::Sum { values } => evaluate_sum(values),
            Node::Product { values } => evaluate_product(values),
            Node::Division { dividend, divisor } => {
                evaluate_division(dividend, divisor)
            }
            Node::Power { base, exponent } => evaluate_power(base, exponent),
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
            Node::Number { value: 2.1 }.evaluate().unwrap(),
            Node::Number { value: 2.1 }
        )
    }

    #[test]
    fn evaluate_negate_number() {
        assert_eq!(
            Node::Negate {
                value: Box::new(Node::Number { value: 2.1 })
            }
            .evaluate()
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
            .evaluate()
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
            .evaluate()
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
            .evaluate()
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
            .evaluate()
            .unwrap(),
            Node::Number { value: 36.0 }
        )
    }
}
