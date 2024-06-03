use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::And;

impl And {
    fn child_requires_parenthesis(node: &Node) -> bool {
        match node {
            Node::Or(_) => true,
            _ => false,
        }
    }
}

impl SerializeNode for And {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        And::new(
            self.values
                .iter()
                .map(|value| value.transform_before_serialization(context))
                .collect(),
        )
        .into()
    }

    fn serialize(&self, context: &crate::core::context::Context) -> String {
        self.values
            .iter()
            .map(|value| {
                if And::child_requires_parenthesis(value) {
                    format!("({})", value.serialize(context))
                } else {
                    value.serialize(context)
                }
            })
            .collect::<Vec<_>>()
            .join(" & ")
    }
}
