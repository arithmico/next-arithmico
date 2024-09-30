use ast::{Boolean, Node};

use crate::{
    error::SerializeNodeError, serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Boolean {
    fn prepare_serialization(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Boolean::new(self.value))
    }
}

impl SerializeNode for Boolean {
    fn serialize(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        if self.value {
            Ok(String::from("true"))
        } else {
            Ok(String::from("false"))
        }
    }
}
