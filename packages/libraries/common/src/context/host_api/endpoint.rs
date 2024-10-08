use std::collections::HashMap;

use ast::Node;

use crate::{EvaluateNodeContext, EvaluateNodeError, Language};

pub type FunctionExecutor = fn(
    arguments: &Vec<Node>,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError>;

pub type ConstantExecutor = fn(context: &EvaluateNodeContext) -> Node;

pub type TranslatedString = HashMap<Language, String>;

#[derive(Debug, Clone, PartialEq)]
pub enum HostEndpoint {
    Function {
        executor: FunctionExecutor,
        arguments: Vec<String>,
        description: TranslatedString,
        module_name: TranslatedString,
        module_id: String,
    },
    Constant {
        executor: ConstantExecutor,
        description: TranslatedString,
        module_name: TranslatedString,
        module_id: String,
    },
}

impl HostEndpoint {
    pub fn get_module_id(&self) -> &str {
        match self {
            HostEndpoint::Function { module_id, .. } => module_id,
            HostEndpoint::Constant { module_id, .. } => module_id,
        }
    }

    pub fn get_module_name(&self) -> &TranslatedString {
        match self {
            HostEndpoint::Function { module_name, .. } => module_name,
            HostEndpoint::Constant { module_name, .. } => module_name,
        }
    }
}
