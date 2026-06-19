use std::collections::HashMap;

use crate::{
    ArgumentMapping, Context, Node,
    core::{EvaluateNodeError, FunctionSignature},
};
use translate_core::Language;

pub type FunctionExecutor =
    fn(&ArgumentMapping, &Context) -> Result<Node, EvaluateNodeError>;

pub type ConstantExecutor = fn(&Context) -> Node;

pub type TranslatedString = HashMap<Language, String>;

#[derive(Debug)]
pub struct EndpointMetadata {
    endpoint_name: String,
    module_id: String,
    module_name: TranslatedString,
    description: TranslatedString,
}

impl EndpointMetadata {
    pub fn new(
        endpoint_name: String,
        module_id: String,
        module_name: TranslatedString,
        description: TranslatedString,
    ) -> Self {
        EndpointMetadata {
            endpoint_name,
            module_id,
            module_name,
            description,
        }
    }

    pub fn endpoint_name(&self) -> &str {
        &self.endpoint_name
    }

    pub fn module_id(&self) -> &str {
        &self.module_id
    }

    pub fn module_name(&self) -> &TranslatedString {
        &self.module_name
    }

    pub fn description(&self) -> &TranslatedString {
        &self.description
    }
}

#[derive(Debug)]
pub enum HostEndpoint {
    Function {
        metadata: EndpointMetadata,
        signature: FunctionSignature,
        executor: FunctionExecutor,
    },
    Constant {
        metadata: EndpointMetadata,
        executor: ConstantExecutor,
    },
}

impl HostEndpoint {
    fn metadata(&self) -> &EndpointMetadata {
        match self {
            HostEndpoint::Function { metadata, .. } => metadata,
            HostEndpoint::Constant { metadata, .. } => metadata,
        }
    }

    pub fn endpoint_name(&self) -> &str {
        &self.metadata().endpoint_name
    }

    pub fn module_id(&self) -> &str {
        &self.metadata().module_id
    }

    pub fn module_name(&self) -> &TranslatedString {
        &self.metadata().module_name
    }
}
