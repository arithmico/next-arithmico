use super::parenthesis::ParenthesesBehavior;
use crate::core::{context::Context, node::Node};

impl Node {
    pub fn serialize(&self, context: &Context) -> String {
        let transformed_node = self.pre_serialize_transform(context);
        transformed_node.serialize_transformed_node(context)
    }

    fn serialize_transformed_node_for_parent(
        &self,
        parent: &Node,
        context: &Context,
    ) -> String {
        match parent.requires_parenthesis(self) {
            ParenthesesBehavior::Optional => {
                self.serialize_transformed_node(context)
            }
            ParenthesesBehavior::Required => {
                format!("({})", self.serialize_transformed_node(context))
            }
        }
    }

    fn serialize_transformed_node(&self, context: &Context) -> String {
        match self {
            Node::Number { value } => {
                if *value == 0.0 {
                    return "0".into();
                }
                if *value < 0.0 {
                    panic!("can not serialize node {}", value);
                }
                let decimal_places =
                    context.settings.get_decimal_places() as usize;
                let serialized_value = format!("{:.1$}", value, decimal_places);
                String::from(
                    serialized_value
                        .trim_end_matches("0")
                        .trim_end_matches("."),
                )
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
                format!(
                    "-{}",
                    value.serialize_transformed_node_for_parent(self, context)
                )
            }
            Node::Sum { values } => {
                values.iter().fold(String::new(), |acc, value| {
                    if acc.is_empty() {
                        value.serialize_transformed_node_for_parent(
                            self, context,
                        )
                    } else {
                        match value {
                            Node::Negate { value } => format!(
                                "{} - {}",
                                acc,
                                value.serialize_transformed_node_for_parent(
                                    self, context
                                )
                            ),
                            _ => format!(
                                "{} + {}",
                                acc,
                                value.serialize_transformed_node_for_parent(
                                    self, context
                                )
                            ),
                        }
                    }
                })
            }

            Node::Product { values } => values
                .iter()
                .map(|value| {
                    value.serialize_transformed_node_for_parent(self, context)
                })
                .collect::<Vec<_>>()
                .join(" * "),
            Node::Division { dividend, divisor } => format!(
                "{} / {}",
                dividend.serialize_transformed_node_for_parent(self, context),
                divisor.serialize_transformed_node_for_parent(self, context)
            ),
            Node::Power { base, exponent } => format!(
                "{}^{}",
                base.serialize_transformed_node_for_parent(self, context),
                exponent.serialize_transformed_node_for_parent(self, context)
            ),
            Node::Vector { values } => format!(
                "[{}]",
                values
                    .iter()
                    .map(|value| value
                        .serialize_transformed_node_for_parent(self, context))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Node::FunctionCall { target, arguments } => format!(
                "{}({})",
                target.serialize_transformed_node_for_parent(self, context),
                arguments
                    .iter()
                    .map(|argument| argument
                        .serialize_transformed_node_for_parent(self, context))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Node::Function {
                arguments,
                expression,
            } => format!(
                "({}) -> {}",
                arguments.join(", "),
                expression.serialize_transformed_node_for_parent(self, context)
            ),
            Node::Definition { symbol, expression } => format!(
                "{} := {}",
                symbol,
                expression.serialize_transformed_node_for_parent(self, context)
            ),
            Node::HostApiFunctionEndpoint { .. } => {
                panic!("cannot serialize function endpoint");
            }
        }
    }

    fn pre_serialize_transform(&self, context: &Context) -> Node {
        match self {
            Node::Number { value } => {
                if *value == 0.0 {
                    return self.clone();
                }
                let decimal_places =
                    context.settings.get_decimal_places() as i32;
                let magnitude = value.abs().log10().round() as i64;
                let magnitude_abs =
                    if magnitude < 0 { -magnitude } else { magnitude };
                if magnitude_abs <= decimal_places as i64 {
                    return if *value < 0.0 {
                        Node::Negate {
                            value: Node::Number { value: value.abs() }.into(),
                        }
                    } else {
                        self.clone()
                    };
                }
                let sign = value.signum();
                let factor = value.abs() * 10_f64.powi(-magnitude as i32);
                let scientific_notation = Node::Product {
                    values: vec![
                        Node::Number { value: factor },
                        Node::Power {
                            base: Node::Number { value: 10.0 }.into(),
                            exponent: if magnitude < 0 {
                                Node::Negate {
                                    value: Node::Number {
                                        value: magnitude.abs() as f64,
                                    }
                                    .into(),
                                }
                            } else {
                                Node::Number {
                                    value: magnitude.abs() as f64,
                                }
                            }
                            .into(),
                        },
                    ],
                };
                if sign > 0.0 {
                    scientific_notation
                } else {
                    Node::Negate {
                        value: scientific_notation.into(),
                    }
                }
            }
            Node::Symbol { .. } => self.clone(),
            Node::Boolean { .. } => self.clone(),
            Node::Negate { value } => Node::Negate {
                value: Box::new(value.pre_serialize_transform(context)),
            },
            Node::Sum { values } => Node::Sum {
                values: values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            },
            Node::Product { values } => Node::Product {
                values: values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            },
            Node::Division { dividend, divisor } => Node::Division {
                dividend: dividend.pre_serialize_transform(context).into(),
                divisor: divisor.pre_serialize_transform(context).into(),
            },
            Node::Power { base, exponent } => Node::Power {
                base: base.pre_serialize_transform(context).into(),
                exponent: exponent.pre_serialize_transform(context).into(),
            },
            Node::Vector { values } => Node::Vector {
                values: values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            },
            Node::FunctionCall { target, arguments } => Node::FunctionCall {
                target: target.pre_serialize_transform(context).into(),
                arguments: arguments
                    .iter()
                    .map(|argument| argument.pre_serialize_transform(context))
                    .collect(),
            },
            Node::Function {
                arguments,
                expression,
            } => Node::Function {
                arguments: arguments.clone(),
                expression: expression.pre_serialize_transform(context).into(),
            },
            Node::Definition { symbol, expression } => Node::Definition {
                symbol: symbol.clone(),
                expression: expression.pre_serialize_transform(context).into(),
            },
            Node::HostApiFunctionEndpoint { name, .. } => {
                Node::Symbol { name: name.into() }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{context::Context, parse::parse::parse_statement};

    fn compare(input: &str, expected: &str) {
        assert_eq!(
            parse_statement(input)
                .unwrap()
                .serialize(&Context::default()),
            expected
        );
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
    fn serialize_number_to_scientific_notation() {
        compare("11234567", "1.12346 * 10^7");
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
