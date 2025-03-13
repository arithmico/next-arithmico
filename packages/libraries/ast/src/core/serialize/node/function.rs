use crate::{Function, Node};

use crate::core::serialize::{
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
            self.signature.clone(),
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
            self.signature
                .argument_names()
                .join(&get_argument_separator(options)),
            self.expression.serialize(options)?
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::{Function, FunctionSignature, NodeType, Number, Sum, Symbol};

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_function_with_no_arguments() {
        assert_eq!(
            serialize_node(
                &Function::new(
                    FunctionSignature::new().add_return_type(NodeType::Any),
                    Number::new(1.)
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "() -> 1"
        );
    }

    #[test]
    fn serialize_function_with_1_argument() {
        assert_eq!(
            serialize_node(
                &Function::new(
                    FunctionSignature::new()
                        .argument("x", |argument| argument
                            .node_type(NodeType::Any))
                        .add_return_type(NodeType::Any),
                    Symbol::new("x")
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(x) -> x"
        );
    }

    #[test]
    fn serialize_function_with_2_arguments() {
        assert_eq!(
            serialize_node(
                &Function::new(
                    FunctionSignature::new()
                        .argument("x", |argument| argument
                            .node_type(NodeType::Any))
                        .argument("y", |argument| argument
                            .node_type(NodeType::Any))
                        .add_return_type(NodeType::Any),
                    Sum::new(vec![Symbol::new("x"), Symbol::new("y"),])
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(x, y) -> x + y"
        );
    }

    #[test]
    fn serialize_nested_functions() {
        assert_eq!(
            serialize_node(
                &Function::new(
                    FunctionSignature::new()
                        .argument("x", |argument| argument
                            .node_type(NodeType::Any))
                        .add_return_type(NodeType::Any),
                    Function::new(
                        FunctionSignature::new()
                            .argument("y", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Sum::new(vec![Symbol::new("x"), Symbol::new("y"),])
                    )
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "(x) -> (y) -> x + y"
        );
    }
}
