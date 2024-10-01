use std::collections::HashMap;

use ast::Node;
use common::Language;

use crate::{context::Context, error::EvaluateNodeError};

pub type FunctionExecutor = fn(
    arguments: &Vec<Node>,
    context: &Context,
) -> Result<Node, EvaluateNodeError>;
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
