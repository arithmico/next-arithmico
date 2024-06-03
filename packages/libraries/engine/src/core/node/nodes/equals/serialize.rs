use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Equals;

impl Equals {
    fn child_requires_parenthesis(node: &Node) -> bool {
        match node {
            Node::And(_) | Node::Or(_) | Node::Equals(_) => true,
            _ => false,
        }
    }

    fn serialize_child(node: &Node, context: &Context) -> String {
        if Equals::child_requires_parenthesis(node) {
            format!("({})", node.serialize(context))
        } else {
            node.serialize(context)
        }
    }
}

impl SerializeNode for Equals {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        format!(
            "{} = {}",
            Equals::serialize_child(&self.left, context),
            Equals::serialize_child(&self.right, context)
        )
    }
}
