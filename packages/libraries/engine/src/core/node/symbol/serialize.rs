use crate::core::{
    Context, Node, NormalizeNode, Serialize, SerializeNodeError, Symbol,
};

impl NormalizeNode for Symbol {
    fn normalize_node(
        &self,
        _context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Symbol::new(&self.name))
    }
}

impl Serialize for Symbol {
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
