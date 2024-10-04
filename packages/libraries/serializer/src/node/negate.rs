use ast::{Negate, Node};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Negate {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        Ok(Negate::new(self.value.prepare_serialization(options)?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Negate(_)
        | Node::Sum(_)
        | Node::Function(_)
        | Node::Definition(_)
        | Node::And(_)
        | Node::Or(_) => true,
        _ => false,
    }
}

impl SerializeNode for Negate {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        Ok(format!(
            "-{}",
            serialize_child(&self.value, options, child_requires_parenthesis)?
        ))
    }
}

#[cfg(test)]
mod tests {

    use ast::{And, Function, Or, Sum, Symbol};

    use crate::serialize_node;

    use super::*;

    #[test]
    fn serialize_negate_symbol() {
        assert_eq!(
            serialize_node(
                &Negate::new(Symbol::new("a")),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "-a"
        );
    }

    #[test]
    fn serialize_nested_negate() {
        assert_eq!(
            serialize_node(
                &Negate::new(Negate::new(Symbol::new("a"))),
                &SerializeNodeOptions::default()
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
                &SerializeNodeOptions::default()
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
                &SerializeNodeOptions::default()
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
                &SerializeNodeOptions::default()
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
                    vec![String::from("x")],
                    Symbol::new("x")
                )),
                &SerializeNodeOptions::default()
            )
            .unwrap(),
            "-((x) -> x)"
        );
    }
}
