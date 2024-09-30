use ast::{Node, Power};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Power {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Power::new(
            self.base.prepare_serialization(options)?,
            self.exponent.prepare_serialization(options)?,
        ))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Product(_)
        | Node::Division(_)
        | Node::Power(_)
        | Node::FunctionCall(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Power {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} ^ {}",
            serialize_child(&self.base, options, child_requires_parenthesis)?,
            serialize_child(
                &self.exponent,
                options,
                child_requires_parenthesis
            )?,
        ))
    }
}
