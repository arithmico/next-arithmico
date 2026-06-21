use node::{Node, Sum};

use crate::core::{Context, Serialize, SerializeNodeError, SerializeUtils};

impl SerializeUtils for Sum {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.normalize_node(context))
            .collect();

        Ok(Sum::new(elements?))
    }

    fn child_requires_parenthesis(
        &self,
        child: &Node,
        _position: usize,
    ) -> bool {
        match child {
            Node::Function(_)
            | Node::And(_)
            | Node::Or(_)
            | Node::Equals(_)
            | Node::LessThan(_)
            | Node::LessThanOrEquals(_)
            | Node::GreaterThan(_)
            | Node::GreaterThanOrEquals(_) => true,
            _ => false,
        }
    }
}

impl Serialize for Sum {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        if self.elements.len() < 2 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let elements: Result<Vec<(String, bool)>, SerializeNodeError> = self
            .elements
            .iter()
            .enumerate()
            .map(|(position, element)| match element {
                Node::Negate(negate) => Ok((
                    self.serialize_child(&negate.value, position, context)?,
                    true,
                )),
                _ => Ok((
                    self.serialize_child(&element, position, context)?,
                    false,
                )),
            })
            .collect();

        Ok(elements?.iter().fold(
            String::new(),
            |acc, (element, is_negative)| {
                if acc.is_empty() && *is_negative {
                    format!("-{}", element)
                } else if acc.is_empty() {
                    element.clone()
                } else if *is_negative {
                    format!("{} - {}", acc, element)
                } else {
                    format!("{} + {}", acc, element)
                }
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use node::{Negate, Sum, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_invalid_sum() {
        assert_eq!(
            serialize_node(
                &Sum::new(vec![Symbol::new("a")]),
                &Context::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_sum_symbol() {
        assert_eq!(
            serialize_node(
                &Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
                &Context::default()
            )
            .unwrap(),
            "a + b"
        );
    }

    #[test]
    fn serialize_nested_sum() {
        assert_eq!(
            serialize_node(
                &Sum::new(vec![
                    Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
                    Sum::new(vec![Symbol::new("c"), Symbol::new("d")]),
                ]),
                &Context::default()
            )
            .unwrap(),
            "a + b + c + d"
        );
    }

    #[test]
    fn serialize_sum_with_negate() {
        assert_eq!(
            serialize_node(
                &Sum::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                    Negate::new(Symbol::new("c"))
                ]),
                &Context::default()
            )
            .unwrap(),
            "a + b - c"
        );
    }

    #[test]
    fn serialize_sum_starting_with_negate() {
        assert_eq!(
            serialize_node(
                &Sum::new(vec![
                    Negate::new(Symbol::new("a")),
                    Symbol::new("b"),
                    Symbol::new("c"),
                ]),
                &Context::default()
            )
            .unwrap(),
            "-a + b + c"
        );
    }
}
