use crate::core::{
    serialize_child, Context, GreaterThanOrEquals, Node, SerializeNode,
    SerializeNodeError, SerializeNodeUtils,
};

impl SerializeNodeUtils for GreaterThanOrEquals {
    fn prepare_serialization(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(GreaterThanOrEquals::new(
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

impl SerializeNode for GreaterThanOrEquals {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} >= {}",
            serialize_child(&self.left, context, child_requires_parenthesis)?,
            serialize_child(&self.right, context, child_requires_parenthesis)?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{And, Equals, Or, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_greater_than_or_equals_with_symbols() {
        assert_eq!(
            serialize_node(
                &GreaterThanOrEquals::new(Symbol::new("x"), Symbol::new("y")),
                &Context::default()
            )
            .unwrap(),
            "x >= y"
        );
    }

    #[test]
    fn serialize_greater_than_or_equals_with_or() {
        assert_eq!(
            serialize_node(
                &GreaterThanOrEquals::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a | b) >= (c | d)"
        );
    }

    #[test]
    fn serialize_greater_than_or_equals_with_and() {
        assert_eq!(
            serialize_node(
                &GreaterThanOrEquals::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a & b) >= (c & d)"
        );
    }

    #[test]
    fn serialize_greater_than_or_equals_with_equals() {
        assert_eq!(
            serialize_node(
                &GreaterThanOrEquals::new(
                    Equals::new(Symbol::new("a"), Symbol::new("b"),),
                    Equals::new(Symbol::new("c"), Symbol::new("d"),),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a = b) >= (c = d)"
        );
    }
}
