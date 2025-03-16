use crate::core::{
    get_argument_separator, Context, Definition, Node, SerializeNode,
    SerializeNodeError, SerializeNodeUtils,
};

impl SerializeNodeUtils for Definition {
    fn prepare_serialization(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Definition::new(
            self.symbol.clone(),
            self.expression.prepare_serialization(context)?,
        ))
    }
}

impl SerializeNode for Definition {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        match self.expression.as_ref() {
            Node::Function(function) => Ok(format!(
                "{}({}) := {}",
                self.symbol,
                function
                    .signature
                    .argument_names()
                    .join(&get_argument_separator(context)),
                function.expression.serialize(context)?
            )),
            _ => Ok(format!(
                "{} := {}",
                self.symbol,
                self.expression.serialize(context)?
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{
        serialize_node, Context, Function, FunctionSignature, NodeType, Number,
        Symbol,
    };

    use super::*;

    #[test]
    fn serialize_define_constant() {
        assert_eq!(
            serialize_node(
                &Definition::new("a", Number::new(1.)),
                &Context::default()
            )
            .unwrap(),
            "a := 1"
        );
    }

    #[test]
    fn serialize_define_function() {
        assert_eq!(
            serialize_node(
                &Definition::new(
                    "f",
                    Function::new(
                        FunctionSignature::new()
                            .argument("x", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Symbol::new("x")
                    )
                ),
                &Context::default()
            )
            .unwrap(),
            "f(x) := x"
        );
    }
}
