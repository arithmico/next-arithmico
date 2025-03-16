use crate::core::{
    Context, Node, SerializeNode, SerializeNodeError, SerializeNodeUtils,
    Symbol,
};

impl SerializeNodeUtils for Symbol {
    fn prepare_serialization(
        &self,
        _context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Symbol::new(&self.name))
    }
}

impl SerializeNode for Symbol {
    fn serialize(
        &self,
        _context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(self.name.clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::core::Symbol;

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_symbol() {
        assert_eq!(
            serialize_node(&Symbol::new("a"), &Context::default()).unwrap(),
            "a"
        );
    }
}
