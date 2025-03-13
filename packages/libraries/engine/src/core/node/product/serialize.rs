use crate::{
    serialize_child, Node, Product, SerializeNode, SerializeNodeError,
    SerializeNodeOptions, SerializeNodeUtils,
};

impl SerializeNodeUtils for Product {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.prepare_serialization(options))
            .collect();

        Ok(Product::new(elements?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Function(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Product {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        if self.elements.len() < 2 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let elements: Result<Vec<String>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| {
                serialize_child(element, options, child_requires_parenthesis)
            })
            .collect();

        Ok(elements?.join(" * "))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        And, Division, Function, FunctionSignature, Negate, NodeType, Or,
        Product, Sum, Symbol,
    };

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_invalid_product() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![Symbol::new("a")]),
                &SerializeNodeOptions::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_product_symbol() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![Symbol::new("a"), Symbol::new("b")]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a * b"
        );
    }

    #[test]
    fn serialize_product_sum() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
                    Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Sum::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a + b) * (c + d)"
        );
    }

    #[test]
    fn serialize_nested_product() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
                    Product::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Product::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a * b * c * d"
        );
    }

    #[test]
    fn serialize_product_negate() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
                    Negate::new(Symbol::new("a")),
                    Negate::new(Symbol::new("b")),
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(-a) * (-b)"
        );
    }

    #[test]
    fn serialize_product_division() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
                    Symbol::new("a"),
                    Division::new(Symbol::new("b"), Symbol::new("c"))
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a * b / c"
        );
    }

    #[test]
    fn serialize_product_and() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
                    And::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d")])
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a & b) * (c & d)"
        );
    }

    #[test]
    fn serialize_product_or() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
                    Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d")])
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a | b) * (c | d)"
        );
    }

    #[test]
    fn serialize_product_function() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![
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
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "((x) -> x) * ((y) -> y)"
        );
    }
}
