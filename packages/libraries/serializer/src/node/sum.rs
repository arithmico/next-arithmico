use ast::{Node, Sum};

use crate::{
    error::SerializeNodeError, parenthesis::serialize_child,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Sum {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.prepare_serialization(options))
            .collect();

        Ok(Sum::new(elements?))
    }
}

fn child_requires_parenthesis(node: &Node) -> bool {
    match node {
        Node::Function(_)
        | Node::And(_)
        | Node::Or(_)
        | Node::Equals(_)
        | Node::LessThan(_)
        | Node::LessThanOrEquals(_)
        | Node::GreaterThan(_)
        | Node::GreaterThanOrEquals(_) => true,
        _ => false,
    }
}

impl SerializeNode for Sum {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        let elements: Result<Vec<(String, bool)>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| match element {
                Node::Negate(negate) => Ok((
                    serialize_child(
                        &negate.value,
                        options,
                        child_requires_parenthesis,
                    )?,
                    true,
                )),
                _ => Ok((
                    serialize_child(
                        &element,
                        options,
                        child_requires_parenthesis,
                    )?,
                    false,
                )),
            })
            .collect();

        Ok(elements?.iter().fold(
            String::new(),
            |acc, (element, is_negative)| {
                if acc.is_empty() && *is_negative {
                    format!("-{}", element)
                } else if *is_negative {
                    format!("{} - {}", acc, element)
                } else {
                    format!("{} + {}", acc, element)
                }
            },
        ))
    }
}
