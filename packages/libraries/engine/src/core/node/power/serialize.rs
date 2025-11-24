use crate::core::{
    serialize_child, Context, Node, NormalizeNode, Power, Serialize,
    SerializeNodeError,
};

impl NormalizeNode for Power {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Power::new(
            self.base.normalize_node(context)?,
            self.exponent.normalize_node(context)?,
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

impl Serialize for Power {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} ^ {}",
            serialize_child(&self.base, context, child_requires_parenthesis)?,
            serialize_child(
                &self.exponent,
                context,
                child_requires_parenthesis
            )?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{
        serialize_node, And, Division, Function, FunctionCall,
        FunctionSignature, Negate, NodeType, Number, Or, Product, Sum, Symbol,
    };

    use super::*;

    #[test]
    fn serialize_power_symbol() {
        assert_eq!(
            serialize_node(
                &Power::new(Symbol::new("a"), Symbol::new("b")),
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
            )
            .unwrap(),
            "(a | b) ^ (c | d)"
        );
    }
}
