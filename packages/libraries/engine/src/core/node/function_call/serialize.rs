use node::{FunctionCall, Node};

use crate::core::{
    get_argument_separator, Context, Serialize, SerializeNodeError,
    SerializeUtils,
};

impl SerializeUtils for FunctionCall {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        let arguments: Result<Vec<Node>, SerializeNodeError> = self
            .arguments
            .iter()
            .map(|element| element.normalize_node(context))
            .collect();

        Ok(FunctionCall::new(
            self.target.normalize_node(context)?,
            arguments?,
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

impl Serialize for FunctionCall {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        let target = match self.target.as_ref() {
            Node::Symbol(symbol) => symbol.name.clone(),
            _ => format!("({})", self.target.serialize(context)?),
        };

        let arguments: Result<Vec<String>, SerializeNodeError> = self
            .arguments
            .iter()
            .map(|element| element.serialize(context))
            .collect();

        Ok(format!(
            "{}({})",
            target,
            arguments?.join(&get_argument_separator(context))
        ))
    }
}

#[cfg(test)]
mod tests {
    use node::{Function, FunctionSignature, NodeType, Sum, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_function_call_no_arguments() {
        assert_eq!(
            serialize_node(
                &FunctionCall::new(Symbol::new("f"), vec![]),
                &Context::default()
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
                &Context::default()
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
                &Context::default()
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
                &Context::default()
            )
            .unwrap(),
            "((x, y) -> x + y)(x, y)"
        );
    }
}
