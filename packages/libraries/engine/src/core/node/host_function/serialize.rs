use node::{HostFunction, Node};

use crate::core::{Context, Serialize, SerializeNodeError, SerializeUtils};

impl SerializeUtils for HostFunction {
    fn normalize_node(
        &self,
        _context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Err(SerializeNodeError::UnsupportedNode)
    }

    fn child_requires_parenthesis(
        &self,
        _child: &Node,
        _position: usize,
    ) -> bool {
        false
    }
}

impl Serialize for HostFunction {
    fn serialize(
        &self,
        _context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Err(SerializeNodeError::UnsupportedNode)
    }
}

#[cfg(test)]
mod tests {

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_host_function() {
        assert_eq!(
            serialize_node(&HostFunction::new("f"), &Context::default()),
            Err(SerializeNodeError::UnsupportedNode)
        );
    }
}
