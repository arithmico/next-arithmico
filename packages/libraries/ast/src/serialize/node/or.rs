use crate::{Node, Or};

use crate::serialize::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Or {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.prepare_serialization(options))
            .collect();

        Ok(Or::new(elements?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        _ => false,
    }
}

impl SerializeNode for Or {
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

        Ok(elements?.join(" | "))
    }
}

#[cfg(test)]
mod tests {

    use crate::Symbol;

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_invalid_or() {
        assert_eq!(
            serialize_node(
                &Or::new(vec![Symbol::new("a")]),
                &SerializeNodeOptions::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_or_2() {
        assert_eq!(
            serialize_node(
                &Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a | b"
        );
    }

    #[test]
    fn serialize_or_3() {
        assert_eq!(
            serialize_node(
                &Or::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                    Symbol::new("c")
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a | b | c"
        );
    }
}
