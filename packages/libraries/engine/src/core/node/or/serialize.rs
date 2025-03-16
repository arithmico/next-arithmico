use crate::core::{
    serialize_child, Context, Node, Or, SerializeNode, SerializeNodeError,
    SerializeNodeUtils,
};

impl SerializeNodeUtils for Or {
    fn prepare_serialization(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.prepare_serialization(context))
            .collect();

        Ok(Or::new(elements?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        _ => false,
    }
}

impl SerializeNode for Or {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        if self.elements.len() < 2 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let elements: Result<Vec<String>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| {
                serialize_child(element, context, child_requires_parenthesis)
            })
            .collect();

        Ok(elements?.join(" | "))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::Symbol;

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_invalid_or() {
        assert_eq!(
            serialize_node(
                &Or::new(vec![Symbol::new("a")]),
                &Context::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_or_2() {
        assert_eq!(
            serialize_node(
                &Or::new(vec![Symbol::new("a"), Symbol::new("b")]),
                &Context::default()
            )
            .unwrap(),
            "a | b"
        );
    }

    #[test]
    fn serialize_or_3() {
        assert_eq!(
            serialize_node(
                &Or::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                    Symbol::new("c")
                ]),
                &Context::default()
            )
            .unwrap(),
            "a | b | c"
        );
    }
}
