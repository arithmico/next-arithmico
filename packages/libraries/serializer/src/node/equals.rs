use ast::{Equals, Node};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Equals {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Equals::new(
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

impl SerializeNode for Equals {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} = {}",
            serialize_child(&self.left, options, child_requires_parenthesis)?,
            serialize_child(&self.right, options, child_requires_parenthesis)?,
        ))
    }
}
