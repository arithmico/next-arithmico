use ast::{Definition, Node};

use crate::{
    argument_separator::get_argument_separator, error::SerializeNodeError,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
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
                function.arguments.join(&get_argument_separator(options)),
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
    use ast::{Function, Number, Symbol};

    use crate::serialize_node;

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
                    Function::new(vec![String::from("x")], Symbol::new("x"))
                ),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "f(x) := x"
        );
    }
}
