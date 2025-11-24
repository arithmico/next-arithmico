use crate::core::{
    get_argument_separator, Context, Function, Node, Serialize,
    SerializeNodeError, SerializeUtils,
};

impl SerializeUtils for Function {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Function::new(
            self.signature.clone(),
            self.expression.normalize_node(context)?,
        ))
    }

    fn child_requires_parenthesis(
        &self,
        _child: &Node,
        _position: usize,
    ) -> bool {
        false
    }
}

impl Serialize for Function {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "({}) -> {}",
            self.signature
                .argument_names()
                .join(&get_argument_separator(context)),
            self.expression.serialize(context)?
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{
        serialize_node, Function, FunctionSignature, NodeType, Number, Sum,
        Symbol,
    };

    use super::*;

    #[test]
    fn serialize_function_with_no_arguments() {
        assert_eq!(
            serialize_node(
                &Function::new(
                    FunctionSignature::new().add_return_type(NodeType::Any),
                    Number::new(1.)
                ),
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
            )
            .unwrap(),
            "(x) -> (y) -> x + y"
        );
    }
}
