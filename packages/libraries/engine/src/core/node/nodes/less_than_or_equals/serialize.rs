use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::LessThanOrEquals;

impl LessThanOrEquals {
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

    fn serialize_child(node: &Node, context: &Context) -> String {
        if LessThanOrEquals::child_requires_parenthesis(node) {
            format!("({})", node.serialize(context))
        } else {
            node.serialize(context)
        }
    }
}

impl SerializeNode for LessThanOrEquals {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        format!(
            "{} <= {}",
            LessThanOrEquals::serialize_child(&self.left, context),
            LessThanOrEquals::serialize_child(&self.right, context)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_less_than() {
        serialization_test("a <= b", "a <= b");
    }
}
