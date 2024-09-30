use crate::{SerializeNodeError, SerializeNodeOptions};

pub(crate) trait SerializeNode {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError>;
}
