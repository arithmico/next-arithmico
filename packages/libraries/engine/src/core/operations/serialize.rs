mod argument_separator;
mod decimal_separator;
mod parenthesis;
mod serialize_node_options;
mod serialize_node_utils;

pub use argument_separator::*;
pub use decimal_separator::*;
pub use parenthesis::*;
pub use serialize_node_options::*;
pub use serialize_node_utils::*;

use crate::core::SerializeNodeError;

pub trait SerializeNode {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError>;
}
