use std::collections::HashMap;

use crate::{
    context::Context, evaluate::NodeEvaluationError, node::Node, Language,
};

pub type FunctionEndpoint = fn(
    arguments: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError>;
pub type ConstantEndpoint = fn(context: &Context) -> Node;

#[derive(Debug, Clone, PartialEq)]
pub enum HostEndpoint {
    Function {
        executor: FunctionEndpoint,
        arguments: Vec<String>,
        description: HashMap<Language, String>,
    },
    Constant {
        executor: ConstantEndpoint,
        description: HashMap<Language, String>,
    },
}
