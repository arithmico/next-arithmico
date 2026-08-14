use std::collections::HashSet;

use node::FunctionSignature;
use translate_core::{Language, RenderedTranslatedMessage};

use crate::{
    ConstantEndpoint, ConstantExecutor, Endpoint, EndpointMetadata,
    FunctionEndpoint, FunctionExecutor,
};

pub struct ApiModule {
    module_name: RenderedTranslatedMessage,
    endpoints: Vec<Endpoint>,
}

impl PartialEq for ApiModule {
    fn eq(&self, other: &Self) -> bool {
        self.module_name == other.module_name
            && self
                .endpoints
                .iter()
                .map(Endpoint::endpoint_name)
                .collect::<HashSet<_>>()
                == other
                    .endpoints
                    .iter()
                    .map(Endpoint::endpoint_name)
                    .collect::<HashSet<_>>()
    }
}

impl ApiModule {
    pub fn builder() -> HostApiModuleBuilderIdStage {
        HostApiModuleBuilderIdStage {}
    }

    pub fn into_endpoints(self) -> Vec<Endpoint> {
        self.endpoints
    }
}

pub struct HostApiModuleBuilderIdStage {}

impl HostApiModuleBuilderIdStage {
    pub fn id(self, id: &str) -> HostApiModuleBuilderNameStage {
        HostApiModuleBuilderNameStage {
            module_id: id.to_string(),
        }
    }
}

pub struct HostApiModuleBuilderNameStage {
    module_id: String,
}

impl HostApiModuleBuilderNameStage {
    pub fn name(
        self,
        language: Language,
        name: &str,
    ) -> HostApiModuleBuilderEndpointsStage {
        let mut module_name = RenderedTranslatedMessage::new();
        module_name.add_message(language, Ok(name.to_string()));

        HostApiModuleBuilderEndpointsStage {
            module_id: self.module_id,
            module_name,
            endpoints: Vec::new(),
        }
    }
}

pub struct HostApiModuleBuilderEndpointsStage {
    module_id: String,
    module_name: RenderedTranslatedMessage,
    endpoints: Vec<Endpoint>,
}

impl HostApiModuleBuilderEndpointsStage {
    pub fn name(
        mut self,
        language: Language,
        name: &str,
    ) -> HostApiModuleBuilderEndpointsStage {
        self.module_name.add_message(language, Ok(name.to_string()));
        self
    }

    pub fn function<F: FunctionEndpoint>(mut self) -> Self {
        let endpoint =
            Endpoint::function::<F>(&self.module_id, self.module_name.clone());
        self.endpoints.push(endpoint);
        self
    }

    pub fn constant<F: ConstantEndpoint>(mut self) -> Self {
        let endpoint =
            Endpoint::constant::<F>(&self.module_id, self.module_name.clone());
        self.endpoints.push(endpoint);
        self
    }

    pub fn endpoints(
        mut self,
        endpoint_list: &[fn(builder: EndpointBuilder) -> Endpoint],
    ) -> HostApiModuleBuilderEndpointsStage {
        for loader in endpoint_list {
            let endpoint = loader(EndpointBuilder {
                module_id: self.module_id.clone(),
                module_name: self.module_name.clone(),
            });

            let name = endpoint.endpoint_name();
            self.assert_endpoint_name_is_unique(name);
            self.endpoints.push(endpoint);
        }

        self
    }

    fn assert_endpoint_name_is_unique(&self, name: &str) -> () {
        if self
            .endpoints
            .iter()
            .any(|host_endpoint| host_endpoint.endpoint_name() == name)
        {
            panic!("duplicate endpoint name: {}", name);
        }
    }

    pub fn build(self) -> ApiModule {
        ApiModule {
            module_name: self.module_name,
            endpoints: self.endpoints,
        }
    }
}

pub struct EndpointBuilder {
    module_id: String,
    module_name: RenderedTranslatedMessage,
}

impl EndpointBuilder {
    pub fn name(self, name: &str) -> EndpointBuilderNameStage {
        EndpointBuilderNameStage {
            module_id: self.module_id,
            module_name: self.module_name,
            endpoint_name: name.to_string(),
            description: RenderedTranslatedMessage::new(),
        }
    }
}

pub struct EndpointBuilderNameStage {
    module_id: String,
    endpoint_name: String,
    module_name: RenderedTranslatedMessage,
    description: RenderedTranslatedMessage,
}

impl EndpointBuilderNameStage {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> EndpointBuilderAdditionalDescriptionsStage {
        self.description
            .add_message(language, Ok(String::from(description)));
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
    endpoint_name: String,
    module_name: RenderedTranslatedMessage,
    description: RenderedTranslatedMessage,
}

impl EndpointBuilderAdditionalDescriptionsStage {
    pub fn description(
        mut self,
        language: Language,
        description: &str,
    ) -> Self {
        self.description
            .add_message(language, Ok(String::from(description)));
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

    pub fn constant(self, executor: ConstantExecutor) -> Endpoint {
        Endpoint::Constant {
            metadata: EndpointMetadata::new(
                self.endpoint_name,
                self.module_id,
                self.module_name,
                self.description,
            ),
            executor,
        }
    }
}

pub struct FunctionEndpointBuilderArgumentsPhase {
    module_id: String,
    endpoint_name: String,
    module_name: RenderedTranslatedMessage,
    description: RenderedTranslatedMessage,
    signature: FunctionSignature,
}

impl FunctionEndpointBuilderArgumentsPhase {
    pub fn executor(self, executor: FunctionExecutor) -> Endpoint {
        Endpoint::Function {
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
