mod argument_separator;
mod decimal_separator;
mod error;
mod node;
mod parenthesis;
mod serialize_node;
mod serialize_node_options;
mod serialize_node_utils;

pub use error::SerializeNodeError;
use serialize_node::SerializeNode;
pub use serialize_node_options::SerializeNodeOptions;
use serialize_node_utils::SerializeNodeUtils;

use crate::Node;

pub fn serialize_node(
    node: &Node,
    options: &SerializeNodeOptions,
) -> Result<String, SerializeNodeError> {
    let transformed_node = node.prepare_serialization(options)?;
    transformed_node.serialize(options)
}
