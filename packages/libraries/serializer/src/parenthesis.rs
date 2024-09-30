use ast::Node;

use crate::{
    serialize_node::SerializeNode, SerializeNodeError, SerializeNodeOptions,
};

pub(crate) fn serialize_child(
    node: &Node,
    options: &SerializeNodeOptions,
    f: impl Fn(&Node) -> bool,
) -> Result<String, SerializeNodeError> {
    let serialized_node = node.serialize(options)?;
    if f(node) {
        Ok(format!("({})", serialized_node))
    } else {
        Ok(serialized_node)
    }
}
