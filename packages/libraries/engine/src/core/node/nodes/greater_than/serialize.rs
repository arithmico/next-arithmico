use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::GreaterThan;

impl GreaterThan {
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
        if GreaterThan::child_requires_parenthesis(node) {
            format!("({})", node.serialize(context))
        } else {
            node.serialize(context)
        }
    }
}

impl SerializeNode for GreaterThan {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        format!(
            "{} > {}",
            GreaterThan::serialize_child(&self.left, context),
            GreaterThan::serialize_child(&self.right, context)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_less_than() {
        serialization_test("a > b", "a > b");
    }
}
