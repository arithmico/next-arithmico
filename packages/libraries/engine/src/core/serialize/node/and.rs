use crate::{
    core::serialize::{
        parenthesis::serialize_child, serialize_node::SerializeNode,
        serialize_node_utils::SerializeNodeUtils,
    },
    And, Node, SerializeNodeError, SerializeNodeOptions,
};

impl SerializeNodeUtils for And {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.prepare_serialization(options))
            .collect();

        Ok(And::new(elements?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for And {
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

        Ok(elements?.join(" & "))
    }
}

#[cfg(test)]
mod tests {

    use crate::{serialize_node, Or, Symbol};

    use super::*;

    #[test]
    fn serialize_invalid_and() {
        assert_eq!(
            serialize_node(
                &And::new(vec![Symbol::new("a")]),
                &SerializeNodeOptions::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_and_2() {
        assert_eq!(
            serialize_node(
                &And::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a & b"
        );
    }

    #[test]
    fn serialize_and_3() {
        assert_eq!(
            serialize_node(
                &And::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                    Symbol::new("c")
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "a & b & c"
        );
    }

    #[test]
    fn serialize_and_with_nested_or() {
        assert_eq!(
            serialize_node(
                &And::new(vec![
                    Or::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(a | b) & (c | d)"
        );
    }
}
