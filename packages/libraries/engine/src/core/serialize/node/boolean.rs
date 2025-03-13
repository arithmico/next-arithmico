use crate::{
    Boolean, Node, SerializeNodeError, SerializeNodeOptions,
    core::serialize::{
        serialize_node::SerializeNode, serialize_node_utils::SerializeNodeUtils,
    },
};

impl SerializeNodeUtils for Boolean {
    fn prepare_serialization(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Boolean::new(self.value))
    }
}

impl SerializeNode for Boolean {
    fn serialize(
        &self,
        _options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        if self.value {
            Ok(String::from("true"))
        } else {
            Ok(String::from("false"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_boolean_true() {
        assert_eq!(
            serialize_node(
                &Boolean::new(true),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "true"
        );
    }

    #[test]
    fn serialize_boolean_false() {
        assert_eq!(
            serialize_node(
                &Boolean::new(false),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "false"
        );
    }
}
