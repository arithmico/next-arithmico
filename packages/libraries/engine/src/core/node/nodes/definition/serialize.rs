use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Definition;

impl SerializeNode for Definition {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        Definition::new(
            self.symbol.clone(),
            self.expression.transform_before_serialization(context),
        )
        .into()
    }

    fn serialize(&self, context: &Context) -> String {
        format!("{} := {}", self.symbol, self.expression.serialize(context))
    }
}
