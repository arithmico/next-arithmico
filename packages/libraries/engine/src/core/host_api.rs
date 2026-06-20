mod endpoint;
mod module;
mod endpoint_builder;

use std::collections::HashMap;

pub use endpoint::*;
pub use module::*;
pub use endpoint_builder::*;

#[derive(Debug)]
pub struct HostApi {
    endpoints: HashMap<String, HostEndpoint>,
}

impl PartialEq for HostApi {
    fn eq(&self, other: &Self) -> bool {
        self.endpoints.keys().collect::<Vec<_>>()
            == other.endpoints.keys().collect::<Vec<_>>()
    }
}

impl HostApi {
    pub fn empty() -> Self {
        Self {
            endpoints: HashMap::new(),
        }
    }

    pub fn builder() -> HostApiBuilder {
        HostApiBuilder::default()
    }

    pub fn endpoint(&self, name: &str) -> Option<&HostEndpoint> {
        self.endpoints.get(name)
    }

    pub fn endpoints(&self) -> &HashMap<String, HostEndpoint> {
        &self.endpoints
    }
}

pub struct HostApiBuilder {
    endpoints: HashMap<String, HostEndpoint>,
}

impl Default for HostApiBuilder {
    fn default() -> Self {
        Self {
            endpoints: Default::default(),
        }
    }
}

impl HostApiBuilder {
    pub fn module(
        mut self,
        feature_flag: bool,
        module_loader: fn() -> HostApiModule,
    ) -> HostApiBuilder {
        if feature_flag {
            let module = module_loader();
            for endpoint in module.into_endpoints() {
                let name = endpoint.endpoint_name().to_string();
                if self.endpoints.contains_key(&name) {
                    panic!("endpoint \"{}\" already exists", name);
                }
                self.endpoints.insert(name, endpoint);
            }
        }
        self
    }

    pub fn build(self) -> HostApi {
        HostApi {
            endpoints: self.endpoints,
        }
    }
}
