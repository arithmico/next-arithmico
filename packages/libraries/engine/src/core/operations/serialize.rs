mod argument_separator;
mod decimal_separator;
mod error;
mod parenthesis;
mod serialize_node_options;
mod serialize_node_utils;

pub use argument_separator::*;
pub use decimal_separator::*;
pub use error::*;
pub use parenthesis::*;
pub use serialize_node_options::*;
pub use serialize_node_utils::*;

pub trait SerializeNode {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError>;
}
