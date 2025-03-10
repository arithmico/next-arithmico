use crate::{FunctionCall, Node};

use crate::serialize::{
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

#[cfg(test)]
mod tests {

    use crate::{Function, FunctionSignature, NodeType, Sum, Symbol};

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_function_call_no_arguments() {
        assert_eq!(
            serialize_node(
                &FunctionCall::new(Symbol::new("f"), vec![]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "f()"
        );
    }

    #[test]
    fn serialize_function_call_1_argument() {
        assert_eq!(
            serialize_node(
                &FunctionCall::new(Symbol::new("f"), vec![Symbol::new("x")]),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "f(x)"
        );
    }

    #[test]
    fn serialize_function_call_2_arguments() {
        assert_eq!(
            serialize_node(
                &FunctionCall::new(
                    Symbol::new("f"),
                    vec![Symbol::new("x"), Symbol::new("y")]
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "f(x, y)"
        );
    }

    #[test]
    fn serialize_in_place_function_call() {
        assert_eq!(
            serialize_node(
                &FunctionCall::new(
                    Function::new(
                        FunctionSignature::new()
                            .argument("x", |argument| argument
                                .node_type(NodeType::Any))
                            .argument("y", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Sum::new(vec![Symbol::new("x"), Symbol::new("y"),])
                    ),
                    vec![Symbol::new("x"), Symbol::new("y")]
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "((x, y) -> x + y)(x, y)"
        );
    }
}
