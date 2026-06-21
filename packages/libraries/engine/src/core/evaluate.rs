use node::Node;

use crate::core::{Context, EvaluateNodeError};

use super::EvaluateNode;

pub fn evaluate_node(
    node: &Node,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    node.evaluate(context)
}
