use crate::context::Context;
use crate::node::Node;

use super::nodes::{
    evaluate_definition, evaluate_division, evaluate_function,
    evaluate_function_call, evaluate_negate, evaluate_number, evaluate_power,
    evaluate_product, evaluate_sum, evaluate_symbol, evaluate_vector,
};
use super::NodeEvaluationError;

impl Node {
    pub fn evaluate(
        &self,
        context: &Context,
    ) -> Result<Node, NodeEvaluationError> {
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
            Node::Function {
                arguments,
                expression,
            } => evaluate_function(arguments, expression, context),
            Node::FunctionCall { target, arguments } => {
                evaluate_function_call(target, arguments, context)
            }
            Node::Definition { symbol, expression } => {
                evaluate_definition(symbol, expression, context)
            }
            Node::Vector { values } => evaluate_vector(values, context),
            _ => Err(NodeEvaluationError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::context::HostApi;

    use super::*;

    #[test]
    fn evaluate_number() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Number { value: 2.1 }.evaluate(&context).unwrap(),
            Node::Number { value: 2.1 }
        )
    }

    #[test]
    fn evaluate_negate_number() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Negate {
                value: Box::new(Node::Number { value: 2.1 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: -2.1 }
        )
    }

    #[test]
    fn evaluate_sum() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Sum {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 3.0 }
        )
    }

    #[test]
    fn evaluate_product() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Product {
                values: vec![
                    Node::Number { value: 3.0 },
                    Node::Number { value: 2.0 }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 6.0 }
        )
    }

    #[test]
    fn evaluate_division() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Division {
                dividend: Box::new(Node::Number { value: 6.0 }),
                divisor: Box::new(Node::Number { value: 2.0 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 3.0 }
        )
    }

    #[test]
    fn evaluate_division_by_zero() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Division {
                dividend: Box::new(Node::Number { value: 6.0 }),
                divisor: Box::new(Node::Number { value: 0.0 })
            }
            .evaluate(&context)
            .err()
            .unwrap(),
            NodeEvaluationError::DivisionByZero
        )
    }

    #[test]
    fn evaluate_power() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
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

    #[test]
    fn evaluate_symbol() {
        let host_api = HostApi::builder().build();
        let mut context = Context::new(host_api.into());
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

    #[test]
    fn evaluate_function() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Function {
                arguments: vec![String::from("x")],
                expression: Box::new(Node::Symbol {
                    name: String::from("x")
                })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Function {
                arguments: vec![String::from("x")],
                expression: Box::new(Node::Symbol {
                    name: String::from("x")
                })
            }
        )
    }

    #[test]
    fn evaluate_function_call() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::FunctionCall {
                target: Box::new(Node::Function {
                    arguments: vec![String::from("x")],
                    expression: Box::new(Node::Symbol {
                        name: String::from("x")
                    })
                }),
                arguments: vec![Node::Number { value: 42.0 }]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 42.0 }
        )
    }

    #[test]
    fn evaluate_definition() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Definition {
                symbol: String::from("test"),
                expression: Box::new(Node::Number { value: 42.0 })
            }
            .evaluate(&context)
            .unwrap(),
            Node::Definition {
                symbol: String::from("test"),
                expression: Box::new(Node::Number { value: 42.0 })
            }
        )
    }

    #[test]
    fn evaluate_vector() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Sum {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                        ]
                    }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            },
        )
    }

    #[test]
    fn evaluate_sum_of_vectors() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Sum {
                values: vec![
                    Node::Vector {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                            Node::Number { value: 3.0 },
                        ]
                    },
                    Node::Vector {
                        values: vec![
                            Node::Number { value: 3.0 },
                            Node::Number { value: 2.0 },
                            Node::Number { value: 1.0 },
                        ]
                    },
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 4.0 },
                    Node::Number { value: 4.0 },
                    Node::Number { value: 4.0 },
                ]
            },
        )
    }

    #[test]
    fn evaluate_sum_of_overflowing_numbers() {
        let host_api = HostApi::builder().build();
        let context = Context::new(host_api.into());
        assert_eq!(
            Node::Sum {
                values: vec![
                    Node::Power {
                        base: Node::Number { value: 10.0 }.into(),
                        exponent: Node::Number { value: 32.0 }.into()
                    },
                    Node::Negate {
                        value: Node::Power {
                            base: Node::Number { value: 10.0 }.into(),
                            exponent: Node::Number { value: 32.0 }.into()
                        }
                        .into(),
                    }
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 0.0 }
        )
    }
}
