use crate::core::Node;

use super::{Context, SerializeNode, SerializeNodeError, SerializeNodeUtils};

pub fn serialize_node(
    node: &Node,
    context: &Context,
) -> Result<String, SerializeNodeError> {
    let transformed_node = node.prepare_serialization(context)?;
    transformed_node.serialize(context)
}
