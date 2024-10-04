use std::collections::HashMap;

use ast::Node;
use common::Language;

use crate::{context::EvaluateNodeContext, error::EvaluateNodeError};

pub type FunctionExecutor = fn(
    arguments: &Vec<Node>,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError>;
pub type ConstantExecutor = fn(context: &EvaluateNodeContext) -> Node;

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
