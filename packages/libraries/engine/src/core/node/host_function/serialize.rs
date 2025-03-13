use crate::{
    HostFunction, Node, SerializeNode, SerializeNodeError,
    SerializeNodeOptions, SerializeNodeUtils,
};

impl SerializeNodeUtils for HostFunction {
    fn prepare_serialization(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Err(SerializeNodeError::UnsupportedNode)
    }
}

impl SerializeNode for HostFunction {
    fn serialize(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Err(SerializeNodeError::UnsupportedNode)
    }
}

#[cfg(test)]
mod tests {

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_host_function() {
        assert_eq!(
            serialize_node(
                &HostFunction::new("f"),
                &SerializeNodeOptions::default()
            ),
            Err(SerializeNodeError::UnsupportedNode)
        );
    }
}
