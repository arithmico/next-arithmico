use ast::{Function, Node};

use crate::{
    argument_separator::get_argument_separator, error::SerializeNodeError,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Function {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Function::new(
            self.arguments.clone(),
            self.expression.prepare_serialization(options)?,
        ))
    }
}

impl SerializeNode for Function {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "({}) -> {}",
            self.arguments.join(&get_argument_separator(options)),
            self.expression.serialize(options)?
        ))
    }
}
