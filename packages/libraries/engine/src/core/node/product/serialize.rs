use crate::core::{
    Context, Node, Product, Serialize, SerializeNodeError, SerializeUtils,
};

impl SerializeUtils for Product {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.normalize_node(context))
            .collect();

        Ok(Product::new(elements?))
    }

    fn child_requires_parenthesis(
        &self,
        child: &Node,
        _position: usize,
    ) -> bool {
        match child {
            Node::Negate(_)
            | Node::Sum(_)
            | Node::Function(_)
            | Node::Definition(_)
            | Node::And(_)
            | Node::Or(_) => true,
            _ => false,
        }
    }
}

impl Serialize for Product {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        if self.elements.len() < 2 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let elements: Result<Vec<String>, SerializeNodeError> = self
            .elements
            .iter()
            .enumerate()
            .map(|(position, element)| {
                self.serialize_child(element, position, context)
            })
            .collect();

        Ok(elements?.join(" * "))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{
        And, Division, Function, FunctionSignature, Negate, NodeType, Or,
        Product, Sum, Symbol,
    };

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_invalid_product() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![Symbol::new("a")]),
                &Context::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_product_symbol() {
        assert_eq!(
            serialize_node(
                &Product::new(vec![Symbol::new("a"), Symbol::new("b")]),
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
            )
            .unwrap(),
            "((x) -> x) * ((y) -> y)"
        );
    }
}
