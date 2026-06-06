use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use crate::{
    ArgumentMapping, Context, Node,
    core::{EvaluateNodeError, FunctionSignature},
};
use translate_core::Language;

pub type FunctionExecutor =
    fn(&ArgumentMapping, &Context) -> Result<Node, EvaluateNodeError>;

pub type ConstantExecutor = fn(&Context) -> Node;

pub type TranslatedString = HashMap<Language, String>;

pub enum HostEndpoint {
    Function {
        endpoint_name: String,
        executor: FunctionExecutor,
        signature: FunctionSignature,
        description: TranslatedString,
        module_name: TranslatedString,
        module_id: String,
    },
    Constant {
        endpoint_name: String,
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
                endpoint_name,
                signature,
                description,
                module_name,
                module_id,
                ..
            } => f
                .debug_struct("HostEndpoint::Function")
                .field("endpoint_name", endpoint_name)
                .field("signature", signature)
                .field("description", description)
                .field("module_name", module_name)
                .field("module_id", module_id)
                .finish(),
            HostEndpoint::Constant {
                endpoint_name,
                description,
                module_name,
                module_id,
                ..
            } => f
                .debug_struct("HostEndpoint::Constant")
                .field("endpoint_name", endpoint_name)
                .field("description", description)
                .field("module_name", module_name)
                .field("module_id", module_id)
                .finish(),
        }
    }
}

impl HostEndpoint {
    pub fn get_endpoint_name(&self) -> &str {
        match self {
            HostEndpoint::Function { endpoint_name, .. } => endpoint_name,
            HostEndpoint::Constant { endpoint_name, .. } => endpoint_name,
        }
    }

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
