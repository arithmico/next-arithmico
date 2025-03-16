use crate::core::Node;

use super::{
    SerializeNode, SerializeNodeError, SerializeNodeOptions, SerializeNodeUtils,
};

pub fn serialize_node(
    node: &Node,
    options: &SerializeNodeOptions,
) -> Result<String, SerializeNodeError> {
    let transformed_node = node.prepare_serialization(options)?;
    transformed_node.serialize(options)
}
