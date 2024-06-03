use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Product;

impl Product {
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

    fn serialize_child(node: &Node, context: &Context) -> String {
        if Product::child_requires_parenthesis(node) {
            format!("({})", node.serialize(context))
        } else {
            node.serialize(context)
        }
    }
}

impl SerializeNode for Product {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        self.values
            .iter()
            .map(|node| Product::serialize_child(node, context))
            .collect::<Vec<_>>()
            .join(" * ")
    }
}
