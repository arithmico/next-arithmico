use crate::{
    serialize::{
        parenthesis::serialize_child, serialize_node::SerializeNode,
        serialize_node_utils::SerializeNodeUtils,
    },
    Division, Node, SerializeNodeError, SerializeNodeOptions,
};

impl SerializeNodeUtils for Division {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Division::new(
            self.dividend.prepare_serialization(options)?,
            self.divisor.prepare_serialization(options)?,
        ))
    }
}

fn dividend_requires_parenthesis(node: &Node) -> bool {
    match *node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Function(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

fn divisor_requires_parenthesis(node: &Node) -> bool {
    match *node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Division(_)
        | Node::Product(_)
        | Node::Function(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Division {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} / {}",
            serialize_child(
                &self.dividend,
                options,
                dividend_requires_parenthesis
            )?,
            serialize_child(
                &self.divisor,
                options,
                divisor_requires_parenthesis
            )?
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        serialize_node, And, Function, FunctionSignature, NodeType, Or,
        Product, Sum, Symbol,
    };

    use super::*;

    #[test]
    fn serialize_division() {
        assert_eq!(
            serialize_node(
                &Division::new(Symbol::new("a"), Symbol::new("b")),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a / b"
        );
    }

    #[test]
    fn serialize_left_nested_division() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    Division::new(Symbol::new("a"), Symbol::new("b"),),
                    Symbol::new("c")
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a / b / c"
        );
    }

    #[test]
    fn serialize_right_nested_division() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    Symbol::new("a"),
                    Division::new(Symbol::new("b"), Symbol::new("c"),)
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a / (b / c)"
        );
    }

    #[test]
    fn serialize_division_with_sum() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    Sum::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Sum::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a + b) / (c + d)"
        );
    }

    #[test]
    fn serialize_division_with_product() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    Product::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Product::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a * b / (c * d)"
        );
    }

    #[test]
    fn serialize_division_with_and() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a & b) / (c & d)"
        );
    }

    #[test]
    fn serialize_division_with_or() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a | b) / (c | d)"
        );
    }

    #[test]
    fn serialize_division_with_function() {
        assert_eq!(
            serialize_node(
                &Division::new(
                    Function::new(
                        FunctionSignature::new()
                            .argument("x", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Symbol::new("x")
                    ),
                    Function::new(
                        FunctionSignature::new()
                            .argument("y", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Symbol::new("y")
                    ),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "((x) -> x) / ((y) -> y)"
        );
    }
}
