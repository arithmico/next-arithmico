use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Or;

impl Or {
    fn child_requires_parenthesis(node: &Node) -> bool {
        match node {
            _ => false,
        }
    }
}

impl SerializeNode for Or {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        self.values
            .iter()
            .map(|value| {
                if Or::child_requires_parenthesis(value) {
                    format!("({})", value.serialize(context))
                } else {
                    value.serialize(context)
                }
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_or_1() {
        serialization_test("a|b|c", "a | b | c");
    }

    #[test]
    fn serialize_or_and_1() {
        serialization_test("a & b | c", "a & b | c");
    }

    #[test]
    fn serialize_or_and_2() {
        serialization_test("a & (b | c)", "a & (b | c)");
    }

    #[test]
    fn serialize_or_and_3() {
        serialization_test("(a & b) | c", "a & b | c");
    }
}
