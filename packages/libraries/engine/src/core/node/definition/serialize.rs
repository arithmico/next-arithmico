use crate::{
    get_argument_separator, Definition, Node, SerializeNode,
    SerializeNodeError, SerializeNodeOptions, SerializeNodeUtils,
};

impl SerializeNodeUtils for Definition {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Definition::new(
            self.symbol.clone(),
            self.expression.prepare_serialization(options)?,
        ))
    }
}

impl SerializeNode for Definition {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        match self.expression.as_ref() {
            Node::Function(function) => Ok(format!(
                "{}({}) := {}",
                self.symbol,
                function
                    .signature
                    .argument_names()
                    .join(&get_argument_separator(options)),
                function.expression.serialize(options)?
            )),
            _ => Ok(format!(
                "{} := {}",
                self.symbol,
                self.expression.serialize(options)?
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        core::serialize::serialize_node, Function, FunctionSignature, NodeType,
        Number, SerializeNodeOptions, Symbol,
    };

    use super::*;

    #[test]
    fn serialize_define_constant() {
        assert_eq!(
            serialize_node(
                &Definition::new("a", Number::new(1.)),
                &SerializeNodeOptions::default()
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
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "f(x) := x"
        );
    }
}
