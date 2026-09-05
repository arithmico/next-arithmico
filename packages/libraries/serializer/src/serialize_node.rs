use node::Node;

use crate::{
    Error, Options, normalize::NormalizeNode,
    serialize_normalized_node::SerializeNormalizedNode, serializer::Serializer,
};

trait Sealed {}

impl Sealed for Node {}

#[allow(private_bounds)]
pub trait SerializeNode: Sealed {
    fn serialize(&self, options: Options) -> Result<String, Error>;
}

impl SerializeNode for Node {
    fn serialize(&self, options: Options) -> Result<String, Error> {
        let normalized = self.clone().normalize(options)?;
        let mut serializer = Serializer::new();
        normalized.serialize_normalized_node(&mut serializer, options)?;
        Ok(serializer.complete())
    }
}

#[cfg(test)]
mod test {
    use common::Language;
    use node::{
        And, Boolean, Definition, Division, Equals, Factorial, Function,
        FunctionCall, FunctionSignature, GreaterThan, GreaterThanOrEquals,
        HostFunction, LessThan, LessThanOrEquals, Negate, NodeType, Number, Or,
        Power, Product, Sum, Symbol, Tensor,
    };

    use super::*;

    #[test]
    fn serialize_number_int() {
        let node = Number::new_node(1.);
        assert_eq!(node.serialize(Default::default()).unwrap(), "1");
    }

    #[test]
    fn serialize_negative_number() {
        let node = Number::new_node(-1.);
        assert_eq!(node.serialize(Default::default()).unwrap(), "-1");
    }

    #[test]
    fn serialize_number_float_dot() {
        let node = Number::new_node(1.23);
        assert_eq!(node.serialize(Default::default()).unwrap(), "1.23");
    }

    #[test]
    fn serialize_number_float_comma() {
        let node = Number::new_node(1.23);
        assert_eq!(
            node.serialize(Options::new(Language::German, Default::default()))
                .unwrap(),
            "1,23"
        );
    }

    #[test]
    fn serialize_number_scientific_notation() {
        let node = Number::new_node(112345678.);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "1.12346 * 10 ^ 8"
        );
    }

    #[test]
    fn serialize_number_scientific_notation_negative() {
        let node = Number::new_node(-112345678.);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "-1.12346 * 10 ^ 8"
        );
    }

    #[test]
    fn serialize_boolean_true() {
        let node = Boolean::new(true);
        assert_eq!(node.serialize(Default::default()).unwrap(), "true");
    }

    #[test]
    fn serialize_boolean_false() {
        let node = Boolean::new(false);
        assert_eq!(node.serialize(Default::default()).unwrap(), "false");
    }

    #[test]
    fn serialize_symbol() {
        let node = Symbol::new("a");
        assert_eq!(node.serialize(Default::default()).unwrap(), "a");
    }

    #[test]
    fn serialize_negate_symbol() {
        let node = Negate::new(Symbol::new("a"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "-a");
    }

    #[test]
    fn serialize_nested_negate() {
        let node = Negate::new(Negate::new(Symbol::new("a")));
        assert_eq!(node.serialize(Default::default()).unwrap(), "-(-a)");
    }

    #[test]
    fn serialize_negate_sum() {
        let node =
            Negate::new(Sum::new(vec![Symbol::new("a"), Symbol::new("b")]));
        assert_eq!(node.serialize(Default::default()).unwrap(), "-(a + b)");
    }

    #[test]
    fn serialize_negate_and() {
        let node =
            Negate::new(And::new(vec![Symbol::new("a"), Symbol::new("b")]));
        assert_eq!(node.serialize(Default::default()).unwrap(), "-(a & b)");
    }

    #[test]
    fn serialize_negate_or() {
        let node =
            Negate::new(Or::new(vec![Symbol::new("a"), Symbol::new("b")]));
        assert_eq!(node.serialize(Default::default()).unwrap(), "-(a | b)");
    }

    #[test]
    fn serialize_negate_function() {
        let node = Negate::new(Function::new(
            FunctionSignature::new()
                .argument("x", |argument| argument.node_type(NodeType::Any))
                .add_return_type(NodeType::Any),
            Symbol::new("x"),
        ));
        assert_eq!(node.serialize(Default::default()).unwrap(), "-((x) -> x)");
    }

    #[test]
    fn serialize_invalid_and() {
        let node = And::new(vec![Symbol::new("a")]);
        assert_eq!(
            node.serialize(Default::default()).unwrap_err(),
            Error::malformed_node(&node)
        );
    }

    #[test]
    fn serialize_and_2() {
        let node = And::new(vec![Symbol::new("a"), Symbol::new("b")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a & b");
    }

    #[test]
    fn serialize_and_3() {
        let node = And::new(vec![
            Symbol::new("a"),
            Symbol::new("b"),
            Symbol::new("c"),
        ]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a & b & c");
    }

    #[test]
    fn serialize_and_with_nested_or() {
        let node = And::new(vec![
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) & (c | d)"
        );
    }

    #[test]
    fn serialize_invalid_or() {
        let node = Or::new(vec![Symbol::new("a")]);
        assert_eq!(
            node.serialize(Default::default()).unwrap_err(),
            Error::malformed_node(&node)
        );
    }

    #[test]
    fn serialize_or_2() {
        let node = Or::new(vec![Symbol::new("a"), Symbol::new("b")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a | b");
    }

    #[test]
    fn serialize_or_3() {
        let node =
            Or::new(vec![Symbol::new("a"), Symbol::new("b"), Symbol::new("c")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a | b | c");
    }

    #[test]
    fn serialize_invalid_sum() {
        let node = Sum::new(vec![Symbol::new("a")]);
        assert_eq!(
            node.serialize(Default::default()).unwrap_err(),
            Error::malformed_node(&node)
        );
    }

    #[test]
    fn serialize_sum_symbol() {
        let node = Sum::new(vec![Symbol::new("a"), Symbol::new("b")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a + b");
    }

    #[test]
    fn serialize_nested_sum() {
        let node = Sum::new(vec![
            Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Sum::new(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "a + b + c + d"
        );
    }

    #[test]
    fn serialize_sum_with_negate() {
        let node = Sum::new(vec![
            Symbol::new("a"),
            Symbol::new("b"),
            Negate::new(Symbol::new("c")),
        ]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a + b - c");
    }

    #[test]
    fn serialize_sum_starting_with_negate() {
        let node = Sum::new(vec![
            Negate::new(Symbol::new("a")),
            Symbol::new("b"),
            Symbol::new("c"),
        ]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "-a + b + c");
    }

    #[test]
    fn serialize_invalid_product() {
        let node = Product::new_node(vec![Symbol::new("a")]);
        assert_eq!(
            node.serialize(Default::default()).unwrap_err(),
            Error::malformed_node(&node)
        );
    }

    #[test]
    fn serialize_product_symbol() {
        let node = Product::new_node(vec![Symbol::new("a"), Symbol::new("b")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a * b");
    }

    #[test]
    fn serialize_product_sum() {
        let node = Product::new_node(vec![
            Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Sum::new(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a + b) * (c + d)"
        );
    }

    #[test]
    fn serialize_nested_product() {
        let node = Product::new_node(vec![
            Product::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            Product::new_node(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "a * b * c * d"
        );
    }

    #[test]
    fn serialize_product_negate() {
        let node = Product::new_node(vec![
            Negate::new(Symbol::new("a")),
            Negate::new(Symbol::new("b")),
        ]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "(-a) * (-b)");
    }

    #[test]
    fn serialize_product_division() {
        let node = Product::new_node(vec![
            Symbol::new("a"),
            Division::new(Symbol::new("b"), Symbol::new("c")),
        ]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "a * b / c");
    }

    #[test]
    fn serialize_product_and() {
        let node = Product::new_node(vec![
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) * (c & d)"
        );
    }

    #[test]
    fn serialize_product_or() {
        let node = Product::new_node(vec![
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) * (c | d)"
        );
    }

    #[test]
    fn serialize_product_function() {
        let node = Product::new_node(vec![
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
            Function::new(
                FunctionSignature::new()
                    .argument("y", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("y"),
            ),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "((x) -> x) * ((y) -> y)"
        );
    }

    #[test]
    fn serialize_power_symbol() {
        let node = Power::new(Symbol::new("a"), Symbol::new("b"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "a ^ b");
    }

    #[test]
    fn serialize_power_number() {
        let node = Power::new(Number::new_node(2.), Number::new_node(3.));
        assert_eq!(node.serialize(Default::default()).unwrap(), "2 ^ 3");
    }

    #[test]
    fn serialize_power_product() {
        let node = Power::new(
            Product::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            Product::new_node(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a * b) ^ (c * d)"
        );
    }

    #[test]
    fn serialize_power_sum() {
        let node = Power::new(
            Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Sum::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a + b) ^ (c + d)"
        );
    }

    #[test]
    fn serialize_power_division() {
        let node = Power::new(
            Division::new(Symbol::new("a"), Symbol::new("b")),
            Division::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a / b) ^ (c / d)"
        );
    }

    #[test]
    fn serialize_nested_power() {
        let node = Power::new(
            Power::new(Symbol::new("a"), Symbol::new("b")),
            Power::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a ^ b) ^ (c ^ d)"
        );
    }

    #[test]
    fn serialize_power_negate() {
        let node = Power::new(
            Negate::new(Symbol::new("a")),
            Negate::new(Symbol::new("b")),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "(-a) ^ (-b)");
    }

    #[test]
    fn serialize_power_function_call() {
        let node = Power::new(
            FunctionCall::new(Symbol::new("f"), vec![Symbol::new("x")]),
            FunctionCall::new(Symbol::new("g"), vec![Symbol::new("x")]),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "f(x) ^ g(x)");
    }

    #[test]
    fn serialize_power_function() {
        let node = Power::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .argument("y", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Sum::new(vec![Symbol::new("x"), Symbol::new("y")]),
            ),
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .argument("y", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Sum::new(vec![Symbol::new("x"), Negate::new(Symbol::new("y"))]),
            ),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "((x, y) -> x + y) ^ ((x, y) -> x - y)"
        );
    }

    #[test]
    fn serialize_power_and() {
        let node = Power::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) ^ (c & d)"
        );
    }

    #[test]
    fn serialize_power_or() {
        let node = Power::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) ^ (c | d)"
        );
    }

    #[test]
    fn serialize_function_call_no_arguments() {
        let node = FunctionCall::new(Symbol::new("f"), vec![]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "f()");
    }

    #[test]
    fn serialize_function_call_1_argument() {
        let node = FunctionCall::new(Symbol::new("f"), vec![Symbol::new("x")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "f(x)");
    }

    #[test]
    fn serialize_function_call_2_arguments() {
        let node = FunctionCall::new(
            Symbol::new("f"),
            vec![Symbol::new("x"), Symbol::new("y")],
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "f(x, y)");
    }

    #[test]
    fn serialize_in_place_function_call() {
        let node = FunctionCall::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .argument("y", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Sum::new(vec![Symbol::new("x"), Symbol::new("y")]),
            ),
            vec![Symbol::new("x"), Symbol::new("y")],
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "((x, y) -> x + y)(x, y)"
        );
    }

    #[test]
    fn serialize_define_constant() {
        let node = Definition::new("a", Number::new_node(1.));
        assert_eq!(node.serialize(Default::default()).unwrap(), "a := 1");
    }

    #[test]
    fn serialize_define_function() {
        let node = Definition::new(
            "f",
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "f(x) := x");
    }

    #[test]
    fn serialize_division() {
        let node = Division::new(Symbol::new("a"), Symbol::new("b"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "a / b");
    }

    #[test]
    fn serialize_left_nested_division() {
        let node = Division::new(
            Division::new(Symbol::new("a"), Symbol::new("b")),
            Symbol::new("c"),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "a / b / c");
    }

    #[test]
    fn serialize_right_nested_division() {
        let node = Division::new(
            Symbol::new("a"),
            Division::new(Symbol::new("b"), Symbol::new("c")),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "a / (b / c)");
    }

    #[test]
    fn serialize_division_with_sum() {
        let node = Division::new(
            Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Sum::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a + b) / (c + d)"
        );
    }

    #[test]
    fn serialize_division_with_product() {
        let node = Division::new(
            Product::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            Product::new_node(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "a * b / (c * d)"
        );
    }

    #[test]
    fn serialize_division_with_and() {
        let node = Division::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) / (c & d)"
        );
    }

    #[test]
    fn serialize_division_with_or() {
        let node = Division::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) / (c | d)"
        );
    }

    #[test]
    fn serialize_division_with_function() {
        let node = Division::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
            Function::new(
                FunctionSignature::new()
                    .argument("y", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("y"),
            ),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "((x) -> x) / ((y) -> y)"
        );
    }

    #[test]
    fn serialize_equals_symbol() {
        let node = Equals::new(Symbol::new("a"), Symbol::new("b"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "a = b");
    }

    #[test]
    fn serialize_nested_equals() {
        let node = Equals::new(
            Equals::new(Symbol::new("a"), Symbol::new("b")),
            Equals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a = b) = (c = d)"
        );
    }

    #[test]
    fn serialize_equals_with_and() {
        let node = Equals::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) = (c & d)"
        );
    }

    #[test]
    fn serialize_equals_with_or() {
        let node = Equals::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) = (c | d)"
        );
    }

    #[test]
    fn serialize_equals_with_greater_than() {
        let node = Equals::new(
            GreaterThan::new(Symbol::new("a"), Symbol::new("b")),
            GreaterThan::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a > b) = (c > d)"
        );
    }

    #[test]
    fn serialize_equals_with_greater_than_or_equals() {
        let node = Equals::new(
            GreaterThanOrEquals::new(Symbol::new("a"), Symbol::new("b")),
            GreaterThanOrEquals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a >= b) = (c >= d)"
        );
    }

    #[test]
    fn serialize_equals_with_less_than() {
        let node = Equals::new(
            LessThan::new(Symbol::new("a"), Symbol::new("b")),
            LessThan::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a < b) = (c < d)"
        );
    }

    #[test]
    fn serialize_equals_with_less_than_or_equals() {
        let node = Equals::new(
            LessThanOrEquals::new(Symbol::new("a"), Symbol::new("b")),
            LessThanOrEquals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a <= b) = (c <= d)"
        );
    }

    #[test]
    fn serialize_greater_than_with_symbols() {
        let node = GreaterThan::new(Symbol::new("x"), Symbol::new("y"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "x > y");
    }

    #[test]
    fn serialize_greater_than_with_or() {
        let node = GreaterThan::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) > (c | d)"
        );
    }

    #[test]
    fn serialize_greater_than_with_and() {
        let node = GreaterThan::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) > (c & d)"
        );
    }

    #[test]
    fn serialize_greater_than_with_equals() {
        let node = GreaterThan::new(
            Equals::new(Symbol::new("a"), Symbol::new("b")),
            Equals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a = b) > (c = d)"
        );
    }

    #[test]
    fn serialize_greater_than_or_equals_with_symbols() {
        let node = GreaterThanOrEquals::new(Symbol::new("x"), Symbol::new("y"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "x >= y");
    }

    #[test]
    fn serialize_greater_than_or_equals_with_or() {
        let node = GreaterThanOrEquals::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) >= (c | d)"
        );
    }

    #[test]
    fn serialize_greater_than_or_equals_with_and() {
        let node = GreaterThanOrEquals::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) >= (c & d)"
        );
    }

    #[test]
    fn serialize_greater_than_or_equals_with_equals() {
        let node = GreaterThanOrEquals::new(
            Equals::new(Symbol::new("a"), Symbol::new("b")),
            Equals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a = b) >= (c = d)"
        );
    }

    #[test]
    fn serialize_less_than_with_symbols() {
        let node = LessThan::new(Symbol::new("x"), Symbol::new("y"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "x < y");
    }

    #[test]
    fn serialize_less_than_with_or() {
        let node = LessThan::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) < (c | d)"
        );
    }

    #[test]
    fn serialize_less_than_with_and() {
        let node = LessThan::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) < (c & d)"
        );
    }

    #[test]
    fn serialize_less_than_with_equals() {
        let node = LessThan::new(
            Equals::new(Symbol::new("a"), Symbol::new("b")),
            Equals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a = b) < (c = d)"
        );
    }

    #[test]
    fn serialize_less_than_or_equals_with_symbols() {
        let node = LessThanOrEquals::new(Symbol::new("x"), Symbol::new("y"));
        assert_eq!(node.serialize(Default::default()).unwrap(), "x <= y");
    }

    #[test]
    fn serialize_less_than_or_equals_with_or() {
        let node = LessThanOrEquals::new(
            Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a | b) <= (c | d)"
        );
    }

    #[test]
    fn serialize_less_than_or_equals_with_and() {
        let node = LessThanOrEquals::new(
            And::new(vec![Symbol::new("a"), Symbol::new("b")]),
            And::new(vec![Symbol::new("c"), Symbol::new("d")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a & b) <= (c & d)"
        );
    }

    #[test]
    fn serialize_less_than_or_equals_with_equals() {
        let node = LessThanOrEquals::new(
            Equals::new(Symbol::new("a"), Symbol::new("b")),
            Equals::new(Symbol::new("c"), Symbol::new("d")),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(a = b) <= (c = d)"
        );
    }

    #[test]
    fn serialize_factorial_number() {
        let node = Factorial::new(Number::new_node(42.0));
        assert_eq!(node.serialize(Default::default()).unwrap(), "42!");
    }

    #[test]
    fn serialize_factorial_product() {
        let node = Factorial::new(Product::new_node(vec![
            Symbol::new("a"),
            Symbol::new("b"),
        ]));
        assert_eq!(node.serialize(Default::default()).unwrap(), "(a * b)!");
    }

    #[test]
    fn serialize_function_with_no_arguments() {
        let node = Function::new(
            FunctionSignature::new().add_return_type(NodeType::Any),
            Number::new_node(1.),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "() -> 1");
    }

    #[test]
    fn serialize_function_with_1_argument() {
        let node = Function::new(
            FunctionSignature::new()
                .argument("x", |argument| argument.node_type(NodeType::Any))
                .add_return_type(NodeType::Any),
            Symbol::new("x"),
        );
        assert_eq!(node.serialize(Default::default()).unwrap(), "(x) -> x");
    }

    #[test]
    fn serialize_function_with_2_arguments() {
        let node = Function::new(
            FunctionSignature::new()
                .argument("x", |argument| argument.node_type(NodeType::Any))
                .argument("y", |argument| argument.node_type(NodeType::Any))
                .add_return_type(NodeType::Any),
            Sum::new(vec![Symbol::new("x"), Symbol::new("y")]),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(x, y) -> x + y"
        );
    }

    #[test]
    fn serialize_nested_functions() {
        let node = Function::new(
            FunctionSignature::new()
                .argument("x", |argument| argument.node_type(NodeType::Any))
                .add_return_type(NodeType::Any),
            Function::new(
                FunctionSignature::new()
                    .argument("y", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Sum::new(vec![Symbol::new("x"), Symbol::new("y")]),
            ),
        );
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "(x) -> ((y) -> x + y)"
        );
    }

    #[test]
    fn serialize_host_function() {
        let node = HostFunction::new("f");
        assert_eq!(
            node.serialize(Default::default()).unwrap_err(),
            Error::unsupported_node(&node)
        );
    }

    #[test]
    fn serialize_empty_tensor() {
        let node = Tensor::new_node(vec![]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "[]");
    }

    #[test]
    fn serialize_tensor_1() {
        let node = Tensor::new_node(vec![Symbol::new("a")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "[a]");
    }

    #[test]
    fn serialize_tensor_2() {
        let node = Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]);
        assert_eq!(node.serialize(Default::default()).unwrap(), "[a, b]");
    }

    #[test]
    fn serialize_nested_tensor_rank_2() {
        let node = Tensor::new_node(vec![
            Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            Tensor::new_node(vec![Symbol::new("c"), Symbol::new("d")]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "[[a, b], [c, d]]"
        );
    }

    #[test]
    fn serialize_nested_tensor_rank_3() {
        let node = Tensor::new_node(vec![
            Tensor::new_node(vec![
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            ]),
            Tensor::new_node(vec![
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            ]),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "[[[a, b], [a, b], [a, b]], [[a, b], [a, b], [a, b]]]"
        );
    }

    #[test]
    fn serialize_mixed_nested_tensor() {
        let node = Tensor::new_node(vec![
            Tensor::new_node(vec![
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
            ]),
            Number::new_node(1.),
        ]);
        assert_eq!(
            node.serialize(Default::default()).unwrap(),
            "[[[a, b], [a, b], [a, b]], 1]"
        );
    }
}
