use std::collections::HashMap;

use crate::Language;

use super::{
    endpoint::{ConstantExecutor, FunctionExecutor, HostEndpoint},
    TranslatedString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HostApiModule {
    name: TranslatedString,
    endpoints: HashMap<String, HostEndpoint>,
}

impl HostApiModule {
    pub fn builder() -> HostApiModuleBuilderIdStage {
        HostApiModuleBuilderIdStage {}
    }

    pub fn get_endpoints(&self) -> &HashMap<String, HostEndpoint> {
        &self.endpoints
    }
}

pub struct HostApiModuleBuilderIdStage {}

impl HostApiModuleBuilderIdStage {
    pub fn id(self, id: &str) -> HostApiModuleBuilderNameStage {
        HostApiModuleBuilderNameStage { id: id.to_string() }
    }
}

pub struct HostApiModuleBuilderNameStage {
    id: String,
}

impl HostApiModuleBuilderNameStage {
    pub fn name(
        self,
        language: Language,
        name: &str,
    ) -> HostApiModuleBuilderEndpointsStage {
        let mut module_name = HashMap::new();
        module_name.insert(language, name.to_string());

        HostApiModuleBuilderEndpointsStage {
            id: self.id,
            name: module_name,
            endpoints: HashMap::new(),
        }
    }
}

pub struct HostApiModuleBuilderEndpointsStage {
    id: String,
    name: TranslatedString,
    endpoints: HashMap<String, HostEndpoint>,
}

impl HostApiModuleBuilderEndpointsStage {
    pub fn name(
        mut self,
        language: Language,
        name: &str,
    ) -> HostApiModuleBuilderEndpointsStage {
        self.name.insert(language, name.to_string());
        self
    }

    pub fn endpoint(
        mut self,
        feature_flag: bool,
        name: &str,
        endpoint: fn(builder: EndpointBuilder) -> HostEndpoint,
    ) -> HostApiModuleBuilderEndpointsStage {
        if feature_flag {
            self.endpoints.insert(
                String::from(name),
                endpoint(EndpointBuilder {
                    module_id: self.id.clone(),
                    module_name: self.name.clone(),
                    description: HashMap::new(),
                }),
            );
        }
        self
    }

    pub fn build(self) -> HostApiModule {
        HostApiModule {
            name: self.name,
            endpoints: self.endpoints,
        }
    }
}

pub struct EndpointBuilder {
    module_id: String,
    module_name: TranslatedString,
    description: TranslatedString,
}

impl EndpointBuilder {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> EndpointBuilderAdditionalDescriptionsStage {
        self.description.insert(language, String::from(description));
        EndpointBuilderAdditionalDescriptionsStage {
            module_id: self.module_id,
            module_name: self.module_name,
            description: self.description,
        }
    }
}

pub struct EndpointBuilderAdditionalDescriptionsStage {
    module_id: String,
    module_name: TranslatedString,
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
        arguments: Vec<&str>,
    ) -> FunctionEndpointBuilderArgumentsPhase {
        FunctionEndpointBuilderArgumentsPhase {
            module_id: self.module_id,
            description: self.description,
            module_name: self.module_name,
            arguments: arguments
                .iter()
                .map(|&argument| String::from(argument))
                .collect(),
        }
    }

    pub fn constant(self, executor: ConstantExecutor) -> HostEndpoint {
        HostEndpoint::Constant {
            module_id: self.module_id,
            module_name: self.module_name,
            description: self.description,
            executor,
        }
    }
}

pub struct FunctionEndpointBuilderArgumentsPhase {
    module_id: String,
    module_name: TranslatedString,
    description: TranslatedString,
    arguments: Vec<String>,
}

impl FunctionEndpointBuilderArgumentsPhase {
    pub fn executor(self, executor: FunctionExecutor) -> HostEndpoint {
        HostEndpoint::Function {
            executor,
            arguments: self.arguments,
            description: self.description,
            module_name: self.module_name,
            module_id: self.module_id,
        }
    }
}
