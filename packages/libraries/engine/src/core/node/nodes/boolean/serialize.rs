use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Boolean;

impl SerializeNode for Boolean {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, _context: &Context) -> String {
        if self.value {
            String::from("true")
        } else {
            String::from("false")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_boolean_true() {
        serialization_test("true", "true");
    }

    #[test]
    fn serialize_boolean_false() {
        serialization_test("false", "false");
    }
}
