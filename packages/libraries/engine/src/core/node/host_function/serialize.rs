use crate::core::{
    Context, HostFunction, Node, SerializeNode, SerializeNodeError,
    SerializeNodeUtils,
};

impl SerializeNodeUtils for HostFunction {
    fn prepare_serialization(
        &self,
        _context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Err(SerializeNodeError::UnsupportedNode)
    }
}

impl SerializeNode for HostFunction {
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
