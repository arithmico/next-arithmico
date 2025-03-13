use crate::{Node, Power};

use crate::core::serialize::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Power {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Power::new(
            self.base.prepare_serialization(options)?,
            self.exponent.prepare_serialization(options)?,
        ))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Product(_)
        | Node::Division(_)
        | Node::Power(_)
        | Node::Function(_)
        | Node::FunctionCall(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Power {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} ^ {}",
            serialize_child(&self.base, options, child_requires_parenthesis)?,
            serialize_child(
                &self.exponent,
                options,
                child_requires_parenthesis
            )?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        And, Division, Function, FunctionCall, FunctionSignature, Negate,
        NodeType, Number, Or, Product, Sum, Symbol,
    };

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_power_symbol() {
        assert_eq!(
            serialize_node(
                &Power::new(Symbol::new("a"), Symbol::new("b")),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a ^ b"
        );
    }

    #[test]
    fn serialize_power_number() {
        assert_eq!(
            serialize_node(
                &Power::new(Number::new(2.), Number::new(3.)),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "2 ^ 3"
        );
    }

    #[test]
    fn serialize_power_product() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Product::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Product::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a * b) ^ (c * d)"
        );
    }

    #[test]
    fn serialize_power_sum() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Sum::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Sum::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a + b) ^ (c + d)"
        );
    }

    #[test]
    fn serialize_power_division() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Division::new(Symbol::new("a"), Symbol::new("b"),),
                    Division::new(Symbol::new("c"), Symbol::new("d"),),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a / b) ^ (c / d)"
        );
    }

    #[test]
    fn serialize_nested_power() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Power::new(Symbol::new("a"), Symbol::new("b"),),
                    Power::new(Symbol::new("c"), Symbol::new("d"),),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a ^ b) ^ (c ^ d)"
        );
    }

    #[test]
    fn serialize_power_negate() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Negate::new(Symbol::new("a")),
                    Negate::new(Symbol::new("b"))
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(-a) ^ (-b)"
        );
    }

    #[test]
    fn serialize_power_function_call() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    FunctionCall::new(Symbol::new("f"), vec![Symbol::new("x")]),
                    FunctionCall::new(Symbol::new("g"), vec![Symbol::new("x")]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(f(x)) ^ (g(x))"
        );
    }

    #[test]
    fn serialize_power_function() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Function::new(
                        FunctionSignature::new()
                            .argument("x", |argument| argument
                                .node_type(NodeType::Any))
                            .argument("y", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Sum::new(vec![Symbol::new("x"), Symbol::new("y"),])
                    ),
                    Function::new(
                        FunctionSignature::new()
                            .argument("x", |argument| argument
                                .node_type(NodeType::Any))
                            .argument("y", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Sum::new(vec![
                            Symbol::new("x"),
                            Negate::new(Symbol::new("y")),
                        ])
                    ),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "((x, y) -> x + y) ^ ((x, y) -> x - y)"
        );
    }

    #[test]
    fn serialize_power_and() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a & b) ^ (c & d)"
        );
    }

    #[test]
    fn serialize_power_or() {
        assert_eq!(
            serialize_node(
                &Power::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a | b) ^ (c | d)"
        );
    }
}
