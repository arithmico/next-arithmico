use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Sum;

impl Sum {
    fn child_requires_parenthesis(node: &Node) -> bool {
        match node {
            Node::Definition(_) | Node::And(_) | Node::Or(_) => true,
            _ => false,
        }
    }

    fn serialize_child(node: &Node, context: &Context) -> String {
        if Sum::child_requires_parenthesis(node) {
            format!("({})", node.serialize(context))
        } else {
            node.serialize(context)
        }
    }
}

impl SerializeNode for Sum {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        self.values.iter().fold(String::new(), |acc, value| {
            if acc.is_empty() {
                Sum::serialize_child(value, context)
            } else {
                match value {
                    Node::Negate(node) => format!(
                        "{} - {}",
                        acc,
                        Sum::serialize_child(&node.value, context)
                    ),
                    _ => format!(
                        "{} + {}",
                        acc,
                        Sum::serialize_child(value, context)
                    ),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_sum() {
        serialization_test("1 + 2 + 3", "1 + 2 + 3");
    }

    #[test]
    fn serialize_sum_with_negate() {
        serialization_test("1 - 2 + 3", "1 - 2 + 3");
    }
}
