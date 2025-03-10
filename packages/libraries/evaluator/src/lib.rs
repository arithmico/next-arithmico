use ast::{EvaluateNodeContext, EvaluateNodeError, Node};
use evaluate::EvaluateNode;

mod evaluate;
mod node;
mod utils;

pub fn evaluate_node(
    node: &Node,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError> {
    node.evaluate(context)
}
