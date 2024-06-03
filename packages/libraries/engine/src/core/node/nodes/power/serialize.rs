use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Power;

impl Power {
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

    fn serialize_child(node: &Node, context: &Context) -> String {
        if Power::child_requires_parenthesis(node) {
            format!("({})", node.serialize(context))
        } else {
            node.serialize(context)
        }
    }
}

impl SerializeNode for Power {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        format!(
            "{}^{}",
            Power::serialize_child(&self.base, context),
            Power::serialize_child(&self.exponent, context),
        )
    }
}
