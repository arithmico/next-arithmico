use ast::Node;
use evaluate::EvaluateNode;

pub use context::{Context, HostApi, HostApiModule, Settings, Stack};
pub use error::EvaluateNodeError;

mod context;
mod error;
mod evaluate;
mod node;

pub fn evaluate_node(
    node: &Node,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    node.evaluate(context)
}
