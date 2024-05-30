use std::collections::HashMap;

use crate::{
    core::{context::Context, node::*},
    language::Language,
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
