use ast::{Division, Node};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Division {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Division::new(
            self.dividend.prepare_serialization(options)?,
            self.divisor.prepare_serialization(options)?,
        ))
    }
}

fn dividend_requires_parenthesis(node: &Node) -> bool {
    match *node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Function(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

fn divisor_requires_parenthesis(node: &Node) -> bool {
    match *node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Division(_)
        | Node::Function(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Division {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{} / {}",
            serialize_child(
                &self.dividend,
                options,
                dividend_requires_parenthesis
            )?,
            serialize_child(
                &self.divisor,
                options,
                divisor_requires_parenthesis
            )?
        ))
    }
}
