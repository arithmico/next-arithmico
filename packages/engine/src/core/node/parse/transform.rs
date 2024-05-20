use crate::core::node::Node;

use super::parser::Rule;
use pest::iterators::{Pair, Pairs};

pub fn transform(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::statement => transform(pair.into_inner().next().unwrap()),
        Rule::number => Node::Number {
            value: pair.as_str().parse().unwrap(),
        },
        Rule::boolean => {
            if pair.as_str() == "true" {
                Node::Boolean { value: true }
            } else {
                Node::Boolean { value: false }
            }
        }
        Rule::symbol => Node::Symbol {
            name: pair.as_str().to_string(),
        },
        Rule::sum => Node::Sum {
            values: pair.into_inner().map(|item| transform(item)).collect(),
        },
        Rule::negate => Node::Negate {
            value: Box::new(transform(pair.into_inner().next().unwrap())),
        },
        Rule::product => Node::Product {
            values: pair.into_inner().map(|item| transform(item)).collect(),
        },
        Rule::division => {
            let mut inner_pairs = pair.into_inner();
            let dividend = transform(inner_pairs.next().unwrap());
            let divisor = transform(inner_pairs.next().unwrap());
            return Node::Division {
                dividend: Box::new(dividend),
                divisor: Box::new(divisor),
            };
        }
        Rule::power => {
            let mut inner_pairs = pair.into_inner();
            let base = transform(inner_pairs.next().unwrap());
            let exponent = transform(inner_pairs.next().unwrap());
            return Node::Power {
                base: Box::new(base),
                exponent: Box::new(exponent),
            };
        }
        Rule::vector => Node::Vector {
            values: pair.into_inner().map(|item| transform(item)).collect(),
        },
        Rule::function_call => {
            let mut inner_pairs = pair.into_inner();

            Node::FunctionCall {
                target: Box::new(transform(
                    next_pair_of_rule(
                        &mut inner_pairs,
                        Rule::function_call_target,
                    )
                    .into_inner()
                    .next()
                    .unwrap(),
                )),
                arguments: next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_call_arguments,
                )
                .into_inner()
                .map(|argument| transform(argument))
                .collect(),
            }
        }
        Rule::function => {
            let mut inner_pairs = pair.into_inner();
            Node::Function {
                arguments: next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_arguments,
                )
                .into_inner()
                .map(|argument| match argument.as_rule() {
                    Rule::symbol => String::from(argument.as_str()),
                    _ => unreachable!(),
                })
                .collect(),
                expression: Box::new(transform(inner_pairs.next().unwrap())),
            }
        }
        Rule::function_definition => {
            let mut inner_pairs = pair.into_inner();
            let symbol = String::from(
                next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_definition_target,
                )
                .into_inner()
                .next()
                .unwrap()
                .as_str(),
            );
            let arguments = next_pair_of_rule(
                &mut inner_pairs,
                Rule::function_definition_arguments,
            )
            .into_inner()
            .map(|argument| String::from(argument.as_str()))
            .collect();
            let expression = transform(inner_pairs.next().unwrap());
            Node::Definition {
                symbol,
                expression: Box::new(Node::Function {
                    arguments,
                    expression: Box::new(expression),
                }),
            }
        }
        Rule::symbol_definition => {
            let mut inner_pairs = pair.into_inner();
            let symbol = String::from(inner_pairs.next().unwrap().as_str());
            let expression = transform(inner_pairs.next().unwrap());
            Node::Definition {
                symbol,
                expression: Box::new(expression),
            }
        }
        _ => unreachable!(),
    }
}

fn next_pair_of_rule<'a>(
    pairs: &'a mut Pairs<Rule>,
    rule: Rule,
) -> Pair<'a, Rule> {
    let pair = pairs.next().unwrap();
    if pair.as_rule() != rule {
        panic!("unexpected rule");
    }
    pair
}

#[cfg(test)]
mod tests {
    use super::super::parse::parse_statement;
    use super::*;

    #[test]
    fn transform_float() {
        let result = parse_statement("2.1").unwrap();
        assert_eq!(result, Node::Number { value: 2.1 });
    }

    #[test]
    fn transform_true() {
        let result = parse_statement("true").unwrap();
        assert_eq!(result, Node::Boolean { value: true });
    }

    #[test]
    fn transform_false() {
        let result = parse_statement("false").unwrap();
        assert_eq!(result, Node::Boolean { value: false });
    }

    #[test]
    fn transform_symbol() {
        let result = parse_statement("hello").unwrap();
        assert_eq!(
            result,
            Node::Symbol {
                name: String::from("hello")
            }
        );
    }

    #[test]
    fn transform_negate() {
        let result = parse_statement("-1").unwrap();
        assert_eq!(
            result,
            Node::Negate {
                value: Box::new(Node::Number { value: 1.0 })
            }
        );
    }

    #[test]
    fn transform_sum() {
        let result = parse_statement("1 + 2 + 3").unwrap();
        assert_eq!(
            result,
            Node::Sum {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            }
        );
    }

    #[test]
    fn transform_sum_with_negate() {
        let result = parse_statement("1 + 2 - 3").unwrap();
        assert_eq!(
            result,
            Node::Sum {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Negate {
                        value: Box::new(Node::Number { value: 3.0 })
                    },
                ]
            }
        );
    }

    #[test]
    fn transform_product() {
        let result = parse_statement("1 * 2").unwrap();
        assert_eq!(
            result,
            Node::Product {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                ]
            }
        );
    }

    #[test]
    fn transform_product_with_inner_sum() {
        let result = parse_statement("(1 + 2) * 3").unwrap();
        assert_eq!(
            result,
            Node::Product {
                values: vec![
                    Node::Sum {
                        values: vec![
                            Node::Number { value: 1.0 },
                            Node::Number { value: 2.0 },
                        ]
                    },
                    Node::Number { value: 3.0 },
                ]
            }
        );
    }

    #[test]
    fn transform_division() {
        let result = parse_statement("1 / 2").unwrap();
        assert_eq!(
            result,
            Node::Division {
                dividend: Box::new(Node::Number { value: 1.0 },),
                divisor: Box::new(Node::Number { value: 2.0 },)
            },
        );
    }

    #[test]
    fn transform_product_with_division() {
        let result = parse_statement("1 / 2 * 3").unwrap();
        assert_eq!(
            result,
            Node::Product {
                values: vec![
                    Node::Division {
                        dividend: Box::new(Node::Number { value: 1.0 },),
                        divisor: Box::new(Node::Number { value: 2.0 },)
                    },
                    Node::Number { value: 3.0 },
                ]
            }
        );
    }

    #[test]
    fn transform_power() {
        let result = parse_statement("2 ^ 3").unwrap();
        assert_eq!(
            result,
            Node::Power {
                base: Box::new(Node::Number { value: 2.0 },),
                exponent: Box::new(Node::Number { value: 3.0 },)
            },
        );
    }

    #[test]
    fn transform_vector() {
        let result = parse_statement("[1, 2, 3]").unwrap();
        assert_eq!(
            result,
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            },
        );
    }

    #[test]
    fn transform_function_call() {
        let result = parse_statement("f(1, 2)").unwrap();
        assert_eq!(
            result,
            Node::FunctionCall {
                target: Box::new(Node::Symbol {
                    name: String::from("f")
                }),
                arguments: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                ]
            }
        );
    }

    #[test]
    fn transform_function() {
        let result = parse_statement("(x, y) -> x + y").unwrap();
        assert_eq!(
            result,
            Node::Function {
                arguments: vec![String::from("x"), String::from("y")],
                expression: Box::new(Node::Sum {
                    values: vec![
                        Node::Symbol {
                            name: String::from("x")
                        },
                        Node::Symbol {
                            name: String::from("y")
                        }
                    ]
                })
            }
        );
    }

    #[test]
    fn transform_function_definition() {
        let result = parse_statement("f(x, y) := x + y").unwrap();
        assert_eq!(
            result,
            Node::Definition {
                symbol: String::from("f"),
                expression: Box::new(Node::Function {
                    arguments: vec![String::from("x"), String::from("y")],
                    expression: Box::new(Node::Sum {
                        values: vec![
                            Node::Symbol {
                                name: String::from("x")
                            },
                            Node::Symbol {
                                name: String::from("y")
                            }
                        ]
                    })
                })
            }
        );
    }

    #[test]
    fn transform_symbol_definition() {
        let result = parse_statement("x := 2").unwrap();
        assert_eq!(
            result,
            Node::Definition {
                symbol: String::from("x"),
                expression: Box::new(Node::Number { value: 2.0 })
            }
        );
    }
}
