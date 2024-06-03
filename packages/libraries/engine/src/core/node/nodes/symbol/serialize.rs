use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Symbol;

impl SerializeNode for Symbol {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, _context: &Context) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_symbol() {
        serialization_test("abc", "abc");
    }
}
