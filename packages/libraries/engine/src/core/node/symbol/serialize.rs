use crate::{
    Node, SerializeNode, SerializeNodeError, SerializeNodeOptions,
    SerializeNodeUtils, Symbol,
};

impl SerializeNodeUtils for Symbol {
    fn prepare_serialization(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Symbol::new(&self.name))
    }
}

impl SerializeNode for Symbol {
    fn serialize(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(self.name.clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::Symbol;

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_symbol() {
        assert_eq!(
            serialize_node(&Symbol::new("a"), &SerializeNodeOptions::default())
                .unwrap(),
            "a"
        );
    }
}
