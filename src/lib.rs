use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod node;
use node::Node;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArithmicoParser;

pub fn parse(input: &str) -> Node {
    let pairs = ArithmicoParser::parse(Rule::program, input)
        .expect("failed to parse")
        .next()
        .unwrap();

    transform(pairs)
}

fn transform(pair: Pair<Rule>) -> Node {
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
            let target_pair = inner_pairs.next().unwrap();
            let target = match target_pair.as_rule() {
                Rule::function_call_target => {
                    transform(target_pair.into_inner().next().unwrap())
                }
                _ => unreachable!(),
            };
            let arguments_pair = inner_pairs.next().unwrap();
            let arguments: Vec<Node> = match arguments_pair.as_rule() {
                Rule::function_call_arguments => arguments_pair
                    .into_inner()
                    .map(|argument| transform(argument))
                    .collect(),
                _ => unreachable!(),
            };

            Node::FunctionCall {
                target: Box::new(target),
                arguments,
            }
        }
        Rule::function => {
            let mut inner_pairs = pair.into_inner();
            let arguments_pair = inner_pairs.next().unwrap();
            let arguments: Vec<String> = match arguments_pair.as_rule() {
                Rule::function_arguments => arguments_pair
                    .into_inner()
                    .map(|argument| match argument.as_rule() {
                        Rule::symbol => String::from(argument.as_str()),
                        _ => unreachable!(),
                    })
                    .collect(),
                _ => unreachable!(),
            };
            let expression = inner_pairs.next().unwrap();
            Node::Function {
                arguments,
                expression: Box::new(transform(expression)),
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_float() {
        let result = parse("2.1");
        assert_eq!(result, Node::Number { value: 2.1 });
    }

    #[test]
    fn parse_true() {
        let result = parse("true");
        assert_eq!(result, Node::Boolean { value: true });
    }

    #[test]
    fn parse_false() {
        let result = parse("false");
        assert_eq!(result, Node::Boolean { value: false });
    }

    #[test]
    fn parse_symbol() {
        let result = parse("hello");
        assert_eq!(
            result,
            Node::Symbol {
                name: String::from("hello")
            }
        );
    }

    #[test]
    fn parse_negate() {
        let result = parse("-1");
        assert_eq!(
            result,
            Node::Negate {
                value: Box::new(Node::Number { value: 1.0 })
            }
        );
    }

    #[test]
    fn parse_sum() {
        let result = parse("1 + 2 + 3");
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
    fn parse_sum_with_negate() {
        let result = parse("1 + 2 - 3");
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
    fn parse_product() {
        let result = parse("1 * 2");
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
    fn parse_product_with_inner_sum() {
        let result = parse("(1 + 2) * 3");
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
    fn parse_division() {
        let result = parse("1 / 2");
        assert_eq!(
            result,
            Node::Division {
                dividend: Box::new(Node::Number { value: 1.0 },),
                divisor: Box::new(Node::Number { value: 2.0 },)
            },
        );
    }

    #[test]
    fn parse_product_with_division() {
        let result = parse("1 / 2 * 3");
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
    fn parse_power() {
        let result = parse("2 ^ 3");
        assert_eq!(
            result,
            Node::Power {
                base: Box::new(Node::Number { value: 2.0 },),
                exponent: Box::new(Node::Number { value: 3.0 },)
            },
        );
    }

    #[test]
    fn parse_vector() {
        let result = parse("[1, 2, 3]");
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
    fn parse_function_call() {
        let result = parse("f(1, 2)");
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
    fn parse_function() {
        let result = parse("(x, y) -> x + y");
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
}
