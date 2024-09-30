use ast::{Negate, Node};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Negate {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Negate::new(self.value.prepare_serialization(options)?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Function(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Negate {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "-{}",
            serialize_child(&self.value, options, child_requires_parenthesis)?
        ))
    }
}
