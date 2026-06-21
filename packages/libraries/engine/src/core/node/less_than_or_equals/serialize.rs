use node::{LessThanOrEquals, Node};

use crate::core::{Context, Serialize, SerializeNodeError, SerializeUtils};

impl SerializeUtils for LessThanOrEquals {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(LessThanOrEquals::new(
            self.left.normalize_node(context)?,
            self.right.normalize_node(context)?,
        ))
    }

    fn child_requires_parenthesis(
        &self,
        child: &Node,
        _position: usize,
    ) -> bool {
        match child {
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
}

impl Serialize for LessThanOrEquals {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} <= {}",
            self.serialize_child(&self.left, 0, context)?,
            self.serialize_child(&self.right, 1, context)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use node::{And, Equals, Or, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_less_than_or_equals_with_symbols() {
        assert_eq!(
            serialize_node(
                &LessThanOrEquals::new(Symbol::new("x"), Symbol::new("y")),
                &Context::default()
            )
            .unwrap(),
            "x <= y"
        );
    }

    #[test]
    fn serialize_less_than_or_equals_with_or() {
        assert_eq!(
            serialize_node(
                &LessThanOrEquals::new(
                    Or::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a | b) <= (c | d)"
        );
    }

    #[test]
    fn serialize_less_than_or_equals_with_and() {
        assert_eq!(
            serialize_node(
                &LessThanOrEquals::new(
                    And::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    And::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a & b) <= (c & d)"
        );
    }

    #[test]
    fn serialize_less_than_or_equals_with_equals() {
        assert_eq!(
            serialize_node(
                &LessThanOrEquals::new(
                    Equals::new(Symbol::new("a"), Symbol::new("b"),),
                    Equals::new(Symbol::new("c"), Symbol::new("d"),),
                ),
                &Context::default()
            )
            .unwrap(),
            "(a = b) <= (c = d)"
        );
    }
}
