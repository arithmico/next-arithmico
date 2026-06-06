use crate::core::{
    And, Context, Node, Serialize, SerializeNodeError, SerializeUtils,
};

impl SerializeUtils for And {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.normalize_node(context))
            .collect();

        Ok(And::new(elements?))
    }

    fn child_requires_parenthesis(
        &self,
        child: &Node,
        _position: usize,
    ) -> bool {
        match child {
            Node::Or(_) => true,
            _ => false,
        }
    }
}

impl Serialize for And {
    fn serialize(
        &self,
        options: &Context,
    ) -> Result<String, SerializeNodeError> {
        if self.elements.len() < 2 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let elements: Result<Vec<String>, SerializeNodeError> = self
            .elements
            .iter()
            .enumerate()
            .map(|(position, element)| {
                self.serialize_child(element, position, options)
            })
            .collect();

        Ok(elements?.join(" & "))
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{Or, Symbol, serialize_node};

    use super::*;

    #[test]
    fn serialize_invalid_and() {
        assert_eq!(
            serialize_node(
                &And::new(vec![Symbol::new("a")]),
                &Context::default()
            ),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_and_2() {
        assert_eq!(
            serialize_node(
                &And::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                &Context::default()
            )
            .unwrap(),
            "a & b"
        );
    }

    #[test]
    fn serialize_and_3() {
        assert_eq!(
            serialize_node(
                &And::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                    Symbol::new("c")
                ]),
                &Context::default()
            )
            .unwrap(),
            "a & b & c"
        );
    }

    #[test]
    fn serialize_and_with_nested_or() {
        assert_eq!(
            serialize_node(
                &And::new(vec![
                    Or::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Or::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ]),
                &Context::default()
            )
            .unwrap(),
            "(a | b) & (c | d)"
        );
    }
}
