use super::parenthesis::ParenthesesBehavior;
use crate::node::Node;

fn serialize_and_wrap_in_parenthesis(node: &Node, parent: &Node) -> String {
    match parent.requires_parenthesis(node) {
        ParenthesesBehavior::Optional => node.serialize(),
        ParenthesesBehavior::Required => format!("({})", node.serialize()),
    }
}

impl Node {
    pub fn serialize(&self) -> String {
        match self {
            Node::Number { value } => {
                if *value < 0.0 {
                    Node::Negate {
                        value: Box::new(Node::Number { value: -*value }),
                    }
                    .serialize()
                } else {
                    value.to_string()
                }
            }
            Node::Symbol { name } => String::from(name),
            Node::Boolean { value } => {
                if *value == true {
                    return String::from("true");
                } else {
                    return String::from("false");
                }
            }
            Node::Negate { value } => {
                format!("-{}", serialize_and_wrap_in_parenthesis(value, self))
            }
            Node::Sum { values } => {
                values.iter().fold(String::new(), |acc, val| {
                    if acc.is_empty() {
                        serialize_and_wrap_in_parenthesis(val, self)
                    } else {
                        match val {
                            Node::Negate { value } => format!(
                                "{} - {}",
                                acc,
                                serialize_and_wrap_in_parenthesis(value, self)
                            ),
                            _ => format!(
                                "{} + {}",
                                acc,
                                serialize_and_wrap_in_parenthesis(val, self)
                            ),
                        }
                    }
                })
            }

            Node::Product { values } => values
                .iter()
                .map(|value| serialize_and_wrap_in_parenthesis(value, self))
                .collect::<Vec<_>>()
                .join(" * "),
            Node::Division { dividend, divisor } => format!(
                "{} / {}",
                serialize_and_wrap_in_parenthesis(dividend, self),
                serialize_and_wrap_in_parenthesis(divisor, self)
            ),
            Node::Power { base, exponent } => format!(
                "{}^{}",
                serialize_and_wrap_in_parenthesis(base, self),
                serialize_and_wrap_in_parenthesis(exponent, self)
            ),
            Node::Vector { values } => format!(
                "[{}]",
                values
                    .iter()
                    .map(|value| serialize_and_wrap_in_parenthesis(value, self))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Node::FunctionCall { target, arguments } => format!(
                "{}({})",
                serialize_and_wrap_in_parenthesis(target, self),
                arguments
                    .iter()
                    .map(|argument| serialize_and_wrap_in_parenthesis(
                        argument, self
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Node::Function {
                arguments,
                expression,
            } => format!(
                "({}) -> {}",
                arguments.join(", "),
                serialize_and_wrap_in_parenthesis(expression, self)
            ),
            Node::Definition { symbol, expression } => format!(
                "{} := {}",
                symbol,
                serialize_and_wrap_in_parenthesis(expression, self)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse::parse_statement;

    fn compare(input: &str, expected: &str) {
        assert_eq!(parse_statement(input).unwrap().serialize(), expected);
    }

    #[test]
    fn serialize_number_integer() {
        compare("2", "2");
    }

    #[test]
    fn serialize_number_float() {
        compare("2.3", "2.3");
    }

    #[test]
    fn serialize_symbol() {
        compare("abc", "abc");
    }

    #[test]
    fn serialize_boolean_true() {
        compare("true", "true");
    }

    #[test]
    fn serialize_boolean_false() {
        compare("false", "false");
    }

    #[test]
    fn serialize_negate() {
        compare("-2", "-2");
    }

    #[test]
    fn serialize_nested_negate() {
        compare("-(-2)", "-(-2)");
    }

    #[test]
    fn serialize_sum() {
        compare("1 + 2 + 3", "1 + 2 + 3");
    }

    #[test]
    fn serialize_sum_with_negate() {
        compare("1 - 2 + 3", "1 - 2 + 3");
    }

    #[test]
    fn serialize_product() {
        compare("1 * 2 * 3", "1 * 2 * 3");
    }

    #[test]
    fn serialize_division() {
        compare("2 / 3", "2 / 3");
    }

    #[test]
    fn serialize_product_with_division() {
        compare("1 * 2 / 3", "1 * 2 / 3");
    }

    #[test]
    fn serialize_power() {
        compare("2 ^ 3", "2^3");
    }

    #[test]
    fn serialize_power_with_sum_and_product() {
        compare("(1+2) ^ (3 * 4)", "(1 + 2)^(3 * 4)");
    }

    #[test]
    fn serialize_nested_power_1() {
        compare("1^(2^3)", "1^(2^3)");
    }

    #[test]
    fn serialize_nested_power_2() {
        compare("(1^2)^3", "(1^2)^3");
    }

    #[test]
    fn serialize_vector() {
        compare("[1,2,3]", "[1, 2, 3]");
    }

    #[test]
    fn serialize_empty_vector() {
        compare("[]", "[]");
    }

    #[test]
    fn serialize_nested_vector() {
        compare("[[1, 2], [3,4]]", "[[1, 2], [3, 4]]");
    }

    #[test]
    fn serialize_function_call_1() {
        compare("func()", "func()");
    }

    #[test]
    fn serialize_function_call_2() {
        compare("f(x,y)", "f(x, y)");
    }

    #[test]
    fn serialize_function_1() {
        compare("() -> 2", "() -> 2");
    }

    #[test]
    fn serialize_function_2() {
        compare("(x) -> x^2", "(x) -> x^2");
    }

    #[test]
    fn serialize_definition() {
        compare("a:=2", "a := 2");
    }
}
