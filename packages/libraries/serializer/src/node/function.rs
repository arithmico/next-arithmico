use ast::{Function, Node};

use crate::{
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
            self.arguments.clone(),
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
            self.arguments.join(&get_argument_separator(options)),
            self.expression.serialize(options)?
        ))
    }
}

#[cfg(test)]
mod tests {

    use ast::{Function, Number, Sum, Symbol};

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_function_with_no_arguments() {
        assert_eq!(
            serialize_node(
                &Function::new(vec![], Number::new(1.)),
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
                &Function::new(vec![String::from("x")], Symbol::new("x")),
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
                    vec![String::from("x"), String::from("y")],
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
                    vec![String::from("x")],
                    Function::new(
                        vec![String::from("y")],
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
