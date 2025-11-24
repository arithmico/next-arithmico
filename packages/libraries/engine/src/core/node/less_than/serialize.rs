use crate::core::{
    serialize_child, Context, LessThan, Node, NormalizeNode, Serialize,
    SerializeNodeError,
};

impl NormalizeNode for LessThan {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(LessThan::new(
            self.left.normalize_node(context)?,
            self.right.normalize_node(context)?,
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

impl Serialize for LessThan {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} < {}",
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
    fn serialize_less_than_with_symbols() {
        assert_eq!(
            serialize_node(
                &LessThan::new(Symbol::new("x"), Symbol::new("y")),
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
            )
            .unwrap(),
            "(a = b) < (c = d)"
        );
    }
}
