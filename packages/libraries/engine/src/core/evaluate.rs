use crate::{EvaluateNodeContext, EvaluateNodeError, Node};

use super::EvaluateNode;

pub fn evaluate_node(
    node: &Node,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError> {
    node.evaluate(context)
}
