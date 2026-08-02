use node::{Factorial, Node};

use crate::{Context, Serialize, SerializeNodeError, core::SerializeUtils};

impl SerializeUtils for Factorial {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Factorial::new(self.value.normalize_node(context)?))
    }

    fn child_requires_parenthesis(
        &self,
        child: &Node,
        _position: usize,
    ) -> bool {
        match child {
            node::Node::Tensor(_)
            | node::Node::Symbol(_)
            | node::Node::FunctionCall(_)
            | node::Node::Number(_)
            | node::Node::Boolean(_)
            | node::Node::Factorial(_) => false,
            _ => true,
        }
    }
}

impl Serialize for Factorial {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "{}!",
            self.serialize_child(&self.value, 0, context)?
        ))
    }
}
