use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use crate::{
    ArgumentsBinding, Context, Node, core::{EvaluateNodeError, FunctionSignature}
};
use translate_core::Language;

pub type FunctionExecutor = Box<
    dyn Fn(&ArgumentsBinding, &Context) -> Result<Node, EvaluateNodeError>
        + Send
        + Sync,
>;

pub type ConstantExecutor = Box<dyn Fn(&Context) -> Node + Send + Sync>;

pub type TranslatedString = HashMap<Language, String>;

pub enum HostEndpoint {
    Function {
        executor: FunctionExecutor,
        signature: FunctionSignature,
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

impl Debug for HostEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HostEndpoint::Function {
                signature,
                description,
                module_name,
                module_id,
                ..
            } => f
                .debug_struct("HostEndpoint::Function")
                .field("signature", signature)
                .field("description", description)
                .field("module_name", module_name)
                .field("module_id", module_id)
                .finish(),
            HostEndpoint::Constant {
                description,
                module_name,
                module_id,
                ..
            } => f
                .debug_struct("HostEndpoint::Constant")
                .field("description", description)
                .field("module_name", module_name)
                .field("module_id", module_id)
                .finish(),
        }
    }
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
