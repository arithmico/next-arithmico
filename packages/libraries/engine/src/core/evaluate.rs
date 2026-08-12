use node::Node;

use crate::core::Context;

use super::EvaluateNode;

pub fn evaluate_node(
    node: &Node,
    context: &Context,
) -> Result<Node, evaluator::Error> {
    node.evaluate(context)
}
