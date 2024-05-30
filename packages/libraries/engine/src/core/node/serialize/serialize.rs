use std::iter::zip;

use super::parenthesis::ParenthesesBehavior;
use crate::core::{
    context::Context,
    node::{nodes::*, Node},
};

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
            Node::Number(Number { value }) => {
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
            Node::Symbol(Symbol { name }) => String::from(name),
            Node::Boolean(Boolean { value }) => {
                if *value == true {
                    return String::from("true");
                } else {
                    return String::from("false");
                }
            }
            Node::Negate(Negate { value }) => {
                format!(
                    "-{}",
                    value.serialize_transformed_node_for_parent(self, context)
                )
            }
            Node::Sum(Sum { values }) => {
                values.iter().fold(String::new(), |acc, value| {
                    if acc.is_empty() {
                        value.serialize_transformed_node_for_parent(
                            self, context,
                        )
                    } else {
                        match value {
                            Node::Negate(Negate { value }) => format!(
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

            Node::Product(Product { values }) => values
                .iter()
                .map(|value| {
                    value.serialize_transformed_node_for_parent(self, context)
                })
                .collect::<Vec<_>>()
                .join(" * "),
            Node::Division(Division { dividend, divisor }) => format!(
                "{} / {}",
                dividend.serialize_transformed_node_for_parent(self, context),
                divisor.serialize_transformed_node_for_parent(self, context)
            ),
            Node::Power(Power { base, exponent }) => format!(
                "{}^{}",
                base.serialize_transformed_node_for_parent(self, context),
                exponent.serialize_transformed_node_for_parent(self, context)
            ),
            Node::Tensor(tensor) => {
                let serialized_elements: Vec<_> = tensor
                    .elements
                    .iter()
                    .map(|element| {
                        element.serialize_transformed_node_for_parent(
                            self, context,
                        )
                    })
                    .collect();
                let rank = tensor.get_rank();
                let mut inner_string = String::new();
                inner_string.push_str(&String::from("[").repeat(rank));
                let mut last_inner_index = 0;
                for (current_inner_index, serialized_element) in
                    serialized_elements.iter().enumerate()
                {
                    let last_outer_index = tensor
                        .convert_to_outer_index(last_inner_index)
                        .unwrap();
                    let current_outer_index = tensor
                        .convert_to_outer_index(current_inner_index)
                        .unwrap();
                    let mut index_delta: Vec<_> =
                        zip(&last_outer_index, &current_outer_index)
                            .map(|(&last_index, &current_index)| {
                                (last_index as isize - current_index as isize)
                                    .abs()
                            })
                            .collect();
                    index_delta.pop();
                    let mut separator = String::new();
                    let sep_count = index_delta.iter().fold(0, |a, b| a + b);
                    for _ in 0..sep_count {
                        separator.push_str("]");
                    }
                    if current_inner_index != 0 {
                        separator.push_str(", ");
                    }
                    for _ in 0..sep_count {
                        separator.push_str("[");
                    }
                    inner_string.push_str(&separator);
                    inner_string.push_str(&serialized_element);
                    last_inner_index = current_inner_index;
                }
                inner_string.push_str(&String::from("]").repeat(rank));
                inner_string
            }
            Node::FunctionCall(FunctionCall { target, arguments }) => format!(
                "{}({})",
                target.serialize_transformed_node_for_parent(self, context),
                arguments
                    .iter()
                    .map(|argument| argument
                        .serialize_transformed_node_for_parent(self, context))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Node::Function(Function {
                arguments,
                expression,
            }) => format!(
                "({}) -> {}",
                arguments.join(", "),
                expression.serialize_transformed_node_for_parent(self, context)
            ),
            Node::Definition(Definition { symbol, expression }) => format!(
                "{} := {}",
                symbol,
                expression.serialize_transformed_node_for_parent(self, context)
            ),
            Node::HostApiFunctionEndpoint { .. } => {
                panic!("cannot serialize function endpoint");
            }
            Node::And(node) => node
                .values
                .iter()
                .map(|value| {
                    value.serialize_transformed_node_for_parent(self, context)
                })
                .collect::<Vec<_>>()
                .join(" & "),

            Node::Or(node) => node
                .values
                .iter()
                .map(|value| {
                    value.serialize_transformed_node_for_parent(self, context)
                })
                .collect::<Vec<_>>()
                .join(" | "),
        }
    }

    fn pre_serialize_transform(&self, context: &Context) -> Node {
        match self {
            Node::Number(Number { value }) => {
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
                        Negate::new(Number::new(value.abs())).into()
                    } else {
                        self.clone()
                    };
                }
                let sign = value.signum();
                let factor = value.abs() * 10_f64.powi(-magnitude as i32);
                let scientific_notation = Product::new(vec![
                    Number::new(factor).into(),
                    Power::new(
                        Number::new(10.0),
                        if magnitude < 0 {
                            Node::from(Negate::new(Number::new(
                                magnitude.abs() as f64,
                            )))
                        } else {
                            Node::from(Number::new(magnitude.abs() as f64))
                        },
                    )
                    .into(),
                ])
                .into();

                if sign > 0.0 {
                    scientific_notation
                } else {
                    Negate::new(scientific_notation).into()
                }
            }
            Node::Symbol(_) => self.clone(),
            Node::Boolean(_) => self.clone(),
            Node::Negate(Negate { value }) => {
                Negate::new(value.pre_serialize_transform(context)).into()
            }
            Node::Sum(Sum { values }) => Sum::new(
                values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            )
            .into(),
            Node::Product(Product { values }) => Product::new(
                values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            )
            .into(),
            Node::Division(Division { dividend, divisor }) => Division::new(
                dividend.pre_serialize_transform(context),
                divisor.pre_serialize_transform(context),
            )
            .into(),
            Node::Power(Power { base, exponent }) => Power::new(
                base.pre_serialize_transform(context),
                exponent.pre_serialize_transform(context),
            )
            .into(),
            Node::Tensor(tensor) => {
                Tensor::new_with_shape(
                    tensor
                        .elements
                        .iter()
                        .map(|value| value.pre_serialize_transform(context))
                        .collect(),
                    tensor.shape.clone(),
                )
                .unwrap()
            }
            .into(),
            Node::FunctionCall(FunctionCall { target, arguments }) => {
                FunctionCall::new(
                    target.pre_serialize_transform(context),
                    arguments
                        .iter()
                        .map(|argument| {
                            argument.pre_serialize_transform(context)
                        })
                        .collect(),
                )
                .into()
            }
            Node::Function(Function {
                arguments,
                expression,
            }) => Function::new(
                arguments.clone(),
                expression.pre_serialize_transform(context),
            )
            .into(),
            Node::Definition(definition) => Definition::new(
                definition.symbol.clone(),
                definition.expression.pre_serialize_transform(context),
            )
            .into(),
            Node::HostApiFunctionEndpoint(endpoint) => {
                Symbol::new(endpoint.name.clone()).into()
            }
            Node::And(node) => And::new(
                node.values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            )
            .into(),
            Node::Or(node) => Or::new(
                node.values
                    .iter()
                    .map(|value| value.pre_serialize_transform(context))
                    .collect(),
            )
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{context::Context, node::parse::parse::parse_statement};

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
        compare("[[1, 2], [3, 4], [5, 6]]", "[[1, 2], [3, 4], [5, 6]]");
        compare(
            "[[[1], [2]], [[3], [4]], [[5], [6]]]",
            "[[[1], [2]], [[3], [4]], [[5], [6]]]",
        );
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

    #[test]
    fn serialize_and() {
        compare("a&b&c", "a & b & c");
    }

    #[test]
    fn serialize_or_1() {
        compare("a|b|c", "a | b | c");
    }

    #[test]
    fn serialize_or_and_1() {
        compare("a & b | c", "a & b | c");
    }

    #[test]
    fn serialize_or_and_2() {
        compare("a & (b | c)", "a & (b | c)");
    }

    #[test]
    fn serialize_or_and_3() {
        compare("(a & b) | c", "a & b | c");
    }
}
