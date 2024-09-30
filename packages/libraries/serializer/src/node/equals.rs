use ast::{Equals, Node};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Equals {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Equals::new(
            self.left.prepare_serialization(options)?,
            self.right.prepare_serialization(options)?,
        ))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::And(_)
        | Node::Or(_)
        | Node::Equals(_)
        | Node::GreaterThan(_)
        | Node::GreaterThanOrEquals(_)
        | Node::LessThan(_)
        | Node::LessThanOrEquals(_) => true,
        _ => false,
    }
}

impl SerializeNode for Equals {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} = {}",
            serialize_child(&self.left, options, child_requires_parenthesis)?,
            serialize_child(&self.right, options, child_requires_parenthesis)?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use ast::{
        And, GreaterThan, GreaterThanOrEquals, LessThan, LessThanOrEquals, Or,
        Symbol,
    };

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_equals_symbol() {
        assert_eq!(
            serialize_node(
                Equals::new(Symbol::new("a"), Symbol::new("b")),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a = b"
        );
    }

    #[test]
    fn serialize_nested_equals() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    Equals::new(Symbol::new("a"), Symbol::new("b")),
                    Equals::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a = b) = (c = d)"
        );
    }

    #[test]
    fn serialize_equals_with_and() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a & b) = (c & d)"
        );
    }

    #[test]
    fn serialize_equals_with_or() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a | b) = (c | d)"
        );
    }

    #[test]
    fn serialize_equals_with_greater_than() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    GreaterThan::new(Symbol::new("a"), Symbol::new("b")),
                    GreaterThan::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a > b) = (c > d)"
        );
    }

    #[test]
    fn serialize_equals_with_greater_than_or_equals() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    GreaterThanOrEquals::new(
                        Symbol::new("a"),
                        Symbol::new("b")
                    ),
                    GreaterThanOrEquals::new(
                        Symbol::new("c"),
                        Symbol::new("d")
                    ),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a >= b) = (c >= d)"
        );
    }

    #[test]
    fn serialize_equals_with_less_than() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    LessThan::new(Symbol::new("a"), Symbol::new("b")),
                    LessThan::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a < b) = (c < d)"
        );
    }

    #[test]
    fn serialize_equals_with_less_than_or_equals() {
        assert_eq!(
            serialize_node(
                Equals::new(
                    LessThanOrEquals::new(Symbol::new("a"), Symbol::new("b")),
                    LessThanOrEquals::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a <= b) = (c <= d)"
        );
    }
}
