use node::{Negate, Node};

use crate::core::{Context, Serialize, SerializeNodeError, SerializeUtils};

impl SerializeUtils for Negate {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Negate::new(self.value.normalize_node(context)?))
    }

    fn child_requires_parenthesis(
        &self,
        child: &Node,
        _position: usize,
    ) -> bool {
        match child {
            Node::Negate(_)
            | Node::Sum(_)
            | Node::Function(_)
            | Node::Definition(_)
            | Node::And(_)
            | Node::Or(_) => true,
            _ => false,
        }
    }
}

impl Serialize for Negate {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "-{}",
            self.serialize_child(&self.value, 0, context)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use node::{And, Function, FunctionSignature, NodeType, Or, Sum, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_negate_symbol() {
        assert_eq!(
            serialize_node(&Negate::new(Symbol::new("a")), &Context::default())
                .unwrap(),
            "-a"
        );
    }

    #[test]
    fn serialize_nested_negate() {
        assert_eq!(
            serialize_node(
                &Negate::new(Negate::new(Symbol::new("a"))),
                &Context::default()
            )
            .unwrap(),
            "-(-a)"
        );
    }

    #[test]
    fn serialize_negate_sum() {
        assert_eq!(
            serialize_node(
                &Negate::new(Sum::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                ])),
                &Context::default()
            )
            .unwrap(),
            "-(a + b)"
        );
    }

    #[test]
    fn serialize_negate_and() {
        assert_eq!(
            serialize_node(
                &Negate::new(And::new(vec![
                    Symbol::new("a"),
                    Symbol::new("b"),
                ])),
                &Context::default()
            )
            .unwrap(),
            "-(a & b)"
        );
    }

    #[test]
    fn serialize_negate_or() {
        assert_eq!(
            serialize_node(
                &Negate::new(Or::new(
                    vec![Symbol::new("a"), Symbol::new("b"),]
                )),
                &Context::default()
            )
            .unwrap(),
            "-(a | b)"
        );
    }

    #[test]
    fn serialize_negate_function() {
        assert_eq!(
            serialize_node(
                &Negate::new(Function::new(
                    FunctionSignature::new()
                        .argument("x", |argument| argument
                            .node_type(NodeType::Any))
                        .add_return_type(NodeType::Any),
                    Symbol::new("x")
                )),
                &Context::default()
            )
            .unwrap(),
            "-((x) -> x)"
        );
    }
}
