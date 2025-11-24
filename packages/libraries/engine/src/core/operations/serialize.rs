mod argument_separator;
mod decimal_separator;
mod parenthesis;
mod serialize_node_utils;

pub use argument_separator::*;
pub use decimal_separator::*;
pub use parenthesis::*;
pub use serialize_node_utils::*;

use crate::{
    core::{Context, SerializeNodeError},
    Node,
};

pub trait Serialize {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError>;
}
