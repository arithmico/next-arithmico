use crate::core::context::Context;

use super::*;
use std::fmt::Debug;

pub trait BaseNode: Into<Node> + PartialEq + Debug + Clone {}
pub trait EvaluateNode: BaseNode {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError>;
}

#[allow(dead_code)]
pub trait SerializeNode: BaseNode {
    fn transform_before_serialization(&self, context: &Context) -> Node;
    fn serialize(&self, context: &Context) -> String;

    fn serialize_with_parenthesis(&self, context: &Context) -> String {
        format!("({})", self.serialize(context))
    }
}
