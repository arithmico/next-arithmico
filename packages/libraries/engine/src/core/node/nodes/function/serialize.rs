use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Function;

impl SerializeNode for Function {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        Function::new(
            self.arguments.clone(),
            self.expression.transform_before_serialization(context),
        )
        .into()
    }

    fn serialize(&self, context: &Context) -> String {
        let arguments = self.arguments.join(", ");
        let expression = self.expression.serialize(context);
        format!("({}) -> {}", arguments, expression)
    }
}
