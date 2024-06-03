use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::FunctionCall;

impl FunctionCall {
    fn target_requires_parenthesis(&self) -> bool {
        match *self.target {
            Node::FunctionCall(_) | Node::Function(_) => true,
            _ => false,
        }
    }
}

impl SerializeNode for FunctionCall {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        FunctionCall::new(
            self.target.transform_before_serialization(context),
            self.arguments
                .iter()
                .map(|argument| {
                    argument.transform_before_serialization(context)
                })
                .collect(),
        )
        .into()
    }

    fn serialize(&self, context: &Context) -> String {
        let target = if self.target_requires_parenthesis() {
            self.target.serialize_with_parenthesis(context)
        } else {
            self.target.serialize(context)
        };
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.serialize(context))
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}({})", target, arguments)
    }
}
