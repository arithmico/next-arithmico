use crate::core::{Context, Node, SerializeNodeError};

use super::SerializeNode;

pub fn serialize_child(
    node: &Node,
    context: &Context,
    f: impl Fn(&Node) -> bool,
) -> Result<String, SerializeNodeError> {
    let serialized_node = node.serialize(context)?;
    if f(node) {
        Ok(format!("({})", serialized_node))
    } else {
        Ok(serialized_node)
    }
}
