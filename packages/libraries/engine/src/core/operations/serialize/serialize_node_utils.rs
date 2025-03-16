use crate::core::Node;

use super::{SerializeNodeError, SerializeNodeOptions};

pub trait SerializeNodeUtils {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError>;
}
