use std::collections::HashMap;

use crate::{
    context::Context, evaluate::NodeEvaluationError, node::Node, Language,
};

pub type FunctionExecutor = fn(
    arguments: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError>;
pub type ConstantExecutor = fn(context: &Context) -> Node;

#[derive(Debug, Clone, PartialEq)]
pub enum HostEndpoint {
    Function {
        executor: FunctionExecutor,
        arguments: Vec<String>,
        description: HashMap<Language, String>,
    },
    Constant {
        executor: ConstantExecutor,
        description: HashMap<Language, String>,
    },
}
