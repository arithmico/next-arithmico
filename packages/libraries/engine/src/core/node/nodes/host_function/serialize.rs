use crate::core::{
    context::Context,
    node::{Node, SerializeNode, Symbol},
};

use super::HostFunction;

impl SerializeNode for HostFunction {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        Symbol::new(self.name.clone()).into()
    }

    fn serialize(&self, _context: &Context) -> String {
        self.name.clone()
    }
}
