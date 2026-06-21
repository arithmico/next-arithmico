use node::Node;

use super::{Context, Serialize, SerializeNodeError, SerializeUtils};

pub fn serialize_node(
    node: &Node,
    context: &Context,
) -> Result<String, SerializeNodeError> {
    let transformed_node = node.normalize_node(context)?;
    transformed_node.serialize(context)
}
