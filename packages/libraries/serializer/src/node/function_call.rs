use ast::{FunctionCall, Node};

use crate::{
    argument_separator::get_argument_separator, error::SerializeNodeError,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for FunctionCall {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        let arguments: Result<Vec<Node>, SerializeNodeError> = self
            .arguments
            .iter()
            .map(|element| element.prepare_serialization(options))
            .collect();

        Ok(FunctionCall::new(
            self.target.prepare_serialization(options)?,
            arguments?,
        ))
    }
}

impl SerializeNode for FunctionCall {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        let target = match self.target.as_ref() {
            Node::Symbol(symbol) => symbol.name.clone(),
            _ => format!("({})", self.target.serialize(options)?),
        };

        let arguments: Result<Vec<String>, SerializeNodeError> = self
            .arguments
            .iter()
            .map(|element| element.serialize(options))
            .collect();

        Ok(format!(
            "{}({})",
            target,
            arguments?.join(&get_argument_separator(options))
        ))
    }
}
