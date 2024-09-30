use ast::{And, Node};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
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
