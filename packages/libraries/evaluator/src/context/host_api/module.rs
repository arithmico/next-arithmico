use std::collections::HashMap;

use common::Language;
use log::info;

use super::endpoint::{ConstantExecutor, FunctionExecutor, HostEndpoint};

#[derive(Debug, Clone, PartialEq)]
pub struct HostApiModule {
    name: String,
    endpoints: HashMap<String, HostEndpoint>,
}

impl HostApiModule {
    pub fn builder() -> HostApiModuleBuilderNameStage {
        HostApiModuleBuilderNameStage {}
    }

    pub fn get_endpoints(&self) -> &HashMap<String, HostEndpoint> {
        &self.endpoints
    }
}

pub struct HostApiModuleBuilderNameStage {}

impl HostApiModuleBuilderNameStage {
    pub fn name(self, name: &str) -> HostApiModuleBuilderEndpointsStage {
        HostApiModuleBuilderEndpointsStage {
            name: String::from(name),
            endpoints: HashMap::new(),
        }
    }
}

pub struct HostApiModuleBuilderEndpointsStage {
    name: String,
    endpoints: HashMap<String, HostEndpoint>,
}

impl HostApiModuleBuilderEndpointsStage {
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
                    description_map: HashMap::new(),
                }),
            );
        } else {
            info!("skip endpoint");
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
    description_map: HashMap<Language, String>,
}

impl EndpointBuilder {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> EndpointBuilderAdditionalDescriptionsStage {
        self.description_map
            .insert(language, String::from(description));
        EndpointBuilderAdditionalDescriptionsStage {
            description_map: self.description_map,
        }
    }
}

pub struct EndpointBuilderAdditionalDescriptionsStage {
    description_map: HashMap<Language, String>,
}

impl EndpointBuilderAdditionalDescriptionsStage {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> Self {
        self.description_map
            .insert(language, String::from(description));
        self
    }

    pub fn function(
        self,
        arguments: Vec<&str>,
    ) -> FunctionEndpointBuilderArgumentsPhase {
        FunctionEndpointBuilderArgumentsPhase {
            description_map: self.description_map,
            arguments: arguments
                .iter()
                .map(|&argument| String::from(argument))
                .collect(),
        }
    }

    pub fn constant(self, executor: ConstantExecutor) -> HostEndpoint {
        HostEndpoint::Constant {
            executor,
            description: self.description_map,
        }
    }
}

pub struct FunctionEndpointBuilderArgumentsPhase {
    description_map: HashMap<Language, String>,
    arguments: Vec<String>,
}

impl FunctionEndpointBuilderArgumentsPhase {
    pub fn executor(self, executor: FunctionExecutor) -> HostEndpoint {
        HostEndpoint::Function {
            executor,
            arguments: self.arguments,
            description: self.description_map,
        }
    }
}
