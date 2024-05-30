use serialize::parenthesis::ParenthesesBehavior;

use crate::core::context::Context;

use super::*;
use std::fmt::Debug;

pub trait BaseNode: Into<Node> + PartialEq + Debug + Clone {}
pub trait EvaluateNode: BaseNode {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError>;
}

#[allow(dead_code)]
pub trait SerializeNode: BaseNode {
    fn child_requires_parenthesis(&self, child: &Node) -> ParenthesesBehavior;
    fn pre_serialize_transform(&self, context: &Context) -> Node;
}
