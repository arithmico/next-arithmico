use crate::Node;

use super::{SerializeNodeError, SerializeNodeOptions};

pub(crate) trait SerializeNodeUtils {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError>;
}
