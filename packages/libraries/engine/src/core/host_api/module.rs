use std::collections::{HashMap, HashSet};

use translate_core::Language;

use crate::core::EndpointBuilder;

use super::{endpoint::HostEndpoint, TranslatedString};

pub struct HostApiModule {
    module_name: TranslatedString,
    endpoints: Vec<HostEndpoint>,
}

impl PartialEq for HostApiModule {
    fn eq(&self, other: &Self) -> bool {
        self.module_name == other.module_name
            && self
                .endpoints
                .iter()
                .map(HostEndpoint::endpoint_name)
                .collect::<HashSet<_>>()
                == other
                    .endpoints
                    .iter()
                    .map(HostEndpoint::endpoint_name)
                    .collect::<HashSet<_>>()
    }
}

impl HostApiModule {
    pub fn builder() -> HostApiModuleBuilderIdStage {
        HostApiModuleBuilderIdStage {}
    }

    pub fn into_endpoints(self) -> Vec<HostEndpoint> {
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
        let mut module_name = HashMap::new();
        module_name.insert(language, name.to_string());

        HostApiModuleBuilderEndpointsStage {
            module_id: self.module_id,
            module_name,
            endpoints: Vec::new(),
        }
    }
}

pub struct HostApiModuleBuilderEndpointsStage {
    module_id: String,
    module_name: TranslatedString,
    endpoints: Vec<HostEndpoint>,
}

impl HostApiModuleBuilderEndpointsStage {
    pub fn name(
        mut self,
        language: Language,
        name: &str,
    ) -> HostApiModuleBuilderEndpointsStage {
        self.module_name.insert(language, name.to_string());
        self
    }

    pub fn endpoints(
        mut self,
        endpoint_list: &[fn(builder: EndpointBuilder) -> HostEndpoint],
    ) -> HostApiModuleBuilderEndpointsStage {
        for loader in endpoint_list {
            let endpoint = loader(EndpointBuilder::new(
                self.module_id.clone(),
                self.module_name.clone(),
            ));

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

    pub fn build(self) -> HostApiModule {
        HostApiModule {
            module_name: self.module_name,
            endpoints: self.endpoints,
        }
    }
}
