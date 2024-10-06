use ast::Node;
use common::{EvaluateNodeContext, EvaluateNodeError};
use evaluate::EvaluateNode;

mod evaluate;
mod node;

pub fn evaluate_node(
    node: &Node,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError> {
    node.evaluate(context)
}
