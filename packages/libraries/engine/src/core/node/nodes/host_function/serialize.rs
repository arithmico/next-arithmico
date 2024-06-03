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

#[cfg(test)]
mod tests {
    use crate::core::{
        context::Context,
        node::{HostFunction, SerializeNode},
    };

    #[test]
    fn serialize_host_function() {
        let context = Context::default();
        assert_eq!(
            HostFunction::new("fooo")
                .transform_before_serialization(&context)
                .serialize(&context),
            "fooo"
        )
    }
}
