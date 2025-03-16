use crate::core::{Node, SerializeNodeError};

use super::SerializeNodeOptions;

pub trait SerializeNodeUtils {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError>;
}
