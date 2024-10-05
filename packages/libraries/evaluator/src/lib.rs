use ast::Node;
use evaluate::EvaluateNode;

pub use context::{
    EndpointBuilder, EvaluateNodeContext, EvaluateNodeOptions, HostApi,
    HostApiModule, HostEndpoint, Stack,
};
pub use error::EvaluateNodeError;

mod context;
mod error;
mod evaluate;
mod node;

pub fn evaluate_node(
    node: &Node,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError> {
    node.evaluate(context)
}
