use ast::{HostFunction, Node};

use crate::{
    error::SerializeNodeError, serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
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
