use crate::core::{
    serialize_child, Context, Equals, Node, SerializeNode, SerializeNodeError,
    SerializeNodeUtils,
};

impl SerializeNodeUtils for Equals {
    fn prepare_serialization(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Equals::new(
            self.left.prepare_serialization(context)?,
            self.right.prepare_serialization(context)?,
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
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} = {}",
            serialize_child(&self.left, context, child_requires_parenthesis)?,
            serialize_child(&self.right, context, child_requires_parenthesis)?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{
        serialize_node, And, GreaterThan, GreaterThanOrEquals, LessThan,
        LessThanOrEquals, Or, Symbol,
    };

    use super::*;

    #[test]
    fn serialize_equals_symbol() {
        assert_eq!(
            serialize_node(
                &Equals::new(Symbol::new("a"), Symbol::new("b")),
                &Context::default()
            )
            .unwrap(),
            "a = b"
        );
    }

    #[test]
    fn serialize_nested_equals() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    Equals::new(Symbol::new("a"), Symbol::new("b")),
                    Equals::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a = b) = (c = d)"
        );
    }

    #[test]
    fn serialize_equals_with_and() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a & b) = (c & d)"
        );
    }

    #[test]
    fn serialize_equals_with_or() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a | b) = (c | d)"
        );
    }

    #[test]
    fn serialize_equals_with_greater_than() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    GreaterThan::new(Symbol::new("a"), Symbol::new("b")),
                    GreaterThan::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a > b) = (c > d)"
        );
    }

    #[test]
    fn serialize_equals_with_greater_than_or_equals() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    GreaterThanOrEquals::new(
                        Symbol::new("a"),
                        Symbol::new("b")
                    ),
                    GreaterThanOrEquals::new(
                        Symbol::new("c"),
                        Symbol::new("d")
                    ),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a >= b) = (c >= d)"
        );
    }

    #[test]
    fn serialize_equals_with_less_than() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    LessThan::new(Symbol::new("a"), Symbol::new("b")),
                    LessThan::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a < b) = (c < d)"
        );
    }

    #[test]
    fn serialize_equals_with_less_than_or_equals() {
        assert_eq!(
            serialize_node(
                &Equals::new(
                    LessThanOrEquals::new(Symbol::new("a"), Symbol::new("b")),
                    LessThanOrEquals::new(Symbol::new("c"), Symbol::new("d")),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a <= b) = (c <= d)"
        );
    }
}
