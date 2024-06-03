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
