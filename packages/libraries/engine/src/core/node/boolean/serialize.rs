use crate::core::{
    Boolean, Context, Node, Serialize, SerializeNodeError, SerializeUtils,
};

impl SerializeUtils for Boolean {
    fn normalize_node(
        &self,
        _context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Boolean::new(self.value))
    }

    fn child_requires_parenthesis(
        &self,
        _child: &Node,
        _position: usize,
    ) -> bool {
        false
    }
}

impl Serialize for Boolean {
    fn serialize(
        &self,
        _context: &Context,
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
    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_boolean_true() {
        assert_eq!(
            serialize_node(&Boolean::new(true), &Context::default()).unwrap(),
            "true"
        );
    }

    #[test]
    fn serialize_boolean_false() {
        assert_eq!(
            serialize_node(&Boolean::new(false), &Context::default()).unwrap(),
            "false"
        );
    }
}
