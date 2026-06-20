use std::collections::HashMap;

use node::{FunctionSignature, NodeType};
use translate_core::Language;

use crate::{
    core::{
        ConstantExecutor, EndpointMetadata, FunctionExecutor, HostEndpoint,
        TranslatedString,
    },
};

pub struct EndpointBuilder {
    module_id: String,
    module_name: TranslatedString,
}

impl EndpointBuilder {
    pub fn new(module_id: String, module_name: TranslatedString) -> Self {
        Self {
            module_id,
            module_name,
        }
    }

    pub fn name(self, name: &str) -> EndpointBuilderNameStage {
        EndpointBuilderNameStage {
            module_id: self.module_id,
            module_name: self.module_name,
            endpoint_name: name.to_string(),
            description: HashMap::new(),
        }
    }
}

pub struct EndpointBuilderNameStage {
    module_id: String,
    module_name: TranslatedString,
    endpoint_name: String,
    description: TranslatedString,
}

impl EndpointBuilderNameStage {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> EndpointBuilderAdditionalDescriptionsStage {
        self.description.insert(language, String::from(description));
        EndpointBuilderAdditionalDescriptionsStage {
            module_id: self.module_id,
            module_name: self.module_name,
            endpoint_name: self.endpoint_name,
            description: self.description,
        }
    }
}

pub struct EndpointBuilderAdditionalDescriptionsStage {
    module_id: String,
    module_name: TranslatedString,
    endpoint_name: String,
    description: TranslatedString,
}

impl EndpointBuilderAdditionalDescriptionsStage {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> Self {
        self.description.insert(language, String::from(description));
        self
    }

    pub fn function(
        self,
        signature: FunctionSignature,
    ) -> FunctionEndpointBuilderArgumentsPhase {
        FunctionEndpointBuilderArgumentsPhase {
            module_id: self.module_id,
            module_name: self.module_name,
            endpoint_name: self.endpoint_name,
            description: self.description,
            signature,
        }
    }

    pub fn node_type(
        self,
        node_type: NodeType,
    ) -> ConstantEndpointBuilderNodeTypeStage {
        ConstantEndpointBuilderNodeTypeStage {
            module_id: self.module_id,
            module_name: self.module_name,
            endpoint_name: self.endpoint_name,
            description: self.description,
            node_type,
        }
    }
}

pub struct FunctionEndpointBuilderArgumentsPhase {
    module_id: String,
    module_name: TranslatedString,
    endpoint_name: String,
    description: TranslatedString,
    signature: FunctionSignature,
}

impl FunctionEndpointBuilderArgumentsPhase {
    pub fn executor(self, executor: FunctionExecutor) -> HostEndpoint {
        HostEndpoint::Function {
            metadata: EndpointMetadata::new(
                self.endpoint_name,
                self.module_id,
                self.module_name,
                self.description,
            ),
            signature: self.signature,
            executor,
        }
    }
}

pub struct ConstantEndpointBuilderNodeTypeStage {
    module_id: String,
    module_name: TranslatedString,
    endpoint_name: String,
    description: TranslatedString,
    node_type: NodeType,
}

impl ConstantEndpointBuilderNodeTypeStage {
    pub fn constant(self, executor: ConstantExecutor) -> HostEndpoint {
        HostEndpoint::Constant {
            metadata: EndpointMetadata::new(
                self.endpoint_name,
                self.module_id,
                self.module_name,
                self.description,
            ),
            node_type: self.node_type,
            executor,
        }
    }
}
