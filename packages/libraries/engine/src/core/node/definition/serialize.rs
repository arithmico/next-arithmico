use crate::core::{
    Context, Definition, Node, Serialize, SerializeNodeError, SerializeUtils,
    get_argument_separator,
};

impl SerializeUtils for Definition {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Definition::new(
            self.symbol.clone(),
            self.expression.normalize_node(context)?,
        ))
    }

    fn child_requires_parenthesis(&self, _: &Node, _poosition: usize) -> bool {
        false
    }
}

impl Serialize for Definition {
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
        Context, Function, FunctionSignature, NodeType, Number, Symbol,
        serialize_node,
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
