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
