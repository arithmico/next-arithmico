use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub type FunctionEndpoint = fn(
    arguments: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError>;
pub type ConstantEndpoint = fn(context: &Context) -> Node;

#[derive(Debug, Clone, PartialEq)]
pub enum HostEndpoint {
    Function(FunctionEndpoint),
    Constant(ConstantEndpoint),
}
