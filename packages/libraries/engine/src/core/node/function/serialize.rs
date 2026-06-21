use std::collections::HashMap;

use node::{Cardinality, Function, Node};
use translate::use_translate;

use crate::core::{
    get_argument_separator, Context, Serialize, SerializeNodeError,
    SerializeUtils,
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

impl Serialize for Cardinality {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, crate::SerializeNodeError> {
        let translate = use_translate();
        let (id, keys) = match self {
            Cardinality::Required => ("engine.cardinality.required", None),
            Cardinality::Optional => ("engine.cardinality.optional", None),
            Cardinality::OptionalWithDefault { default } => {
                let mut keys = HashMap::new();
                let default_string = default.serialize(context)?;
                keys.insert("default".to_string(), default_string);

                ("engine.cardinality.default", Some(keys))
            }
            Cardinality::Multiple { min, max } => {
                let mut keys = HashMap::new();
                keys.insert("min".to_string(), min.to_string());

                match max {
                    Some(max) => {
                        keys.insert("max".to_string(), max.to_string());
                        ("engine.cardinality.range", Some(keys))
                    }
                    None => ("engine.cardinality.range.open", Some(keys)),
                }
            }
        };

        translate(id, keys).map_err(|_| crate::SerializeNodeError::InvalidNode)
    }
}

#[cfg(test)]
mod tests {
    use node::{Function, FunctionSignature, NodeType, Number, Sum, Symbol};

    use crate::core::serialize_node;

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
