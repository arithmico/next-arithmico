use crate::core::{
    serialize_child, LessThan, Node, SerializeNode, SerializeNodeError,
    SerializeNodeOptions, SerializeNodeUtils,
};

impl SerializeNodeUtils for LessThan {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(LessThan::new(
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

impl SerializeNode for LessThan {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} < {}",
            serialize_child(&self.left, options, child_requires_parenthesis)?,
            serialize_child(&self.right, options, child_requires_parenthesis)?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{And, Equals, Or, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_less_than_with_symbols() {
        assert_eq!(
            serialize_node(
                &LessThan::new(Symbol::new("x"), Symbol::new("y")),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "x < y"
        );
    }

    #[test]
    fn serialize_less_than_with_or() {
        assert_eq!(
            serialize_node(
                &LessThan::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a | b) < (c | d)"
        );
    }

    #[test]
    fn serialize_less_than_with_and() {
        assert_eq!(
            serialize_node(
                &LessThan::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a & b) < (c & d)"
        );
    }

    #[test]
    fn serialize_less_than_with_equals() {
        assert_eq!(
            serialize_node(
                &LessThan::new(
                    Equals::new(Symbol::new("a"), Symbol::new("b"),),
                    Equals::new(Symbol::new("c"), Symbol::new("d"),),
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a = b) < (c = d)"
        );
    }
}
