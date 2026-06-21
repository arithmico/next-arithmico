use node::{Node, Symbol};

use crate::core::{Context, Serialize, SerializeNodeError, SerializeUtils};

impl SerializeUtils for Symbol {
    fn normalize_node(
        &self,
        _context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Symbol::new(&self.name))
    }

    fn child_requires_parenthesis(
        &self,
        _child: &Node,
        _position: usize,
    ) -> bool {
        false
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
