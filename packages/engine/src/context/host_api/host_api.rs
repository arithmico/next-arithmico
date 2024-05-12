use std::collections::HashMap;

use super::host_endpoint::{ConstantEndpoint, FunctionEndpoint, HostEndpoint};

#[derive(Debug, Clone, PartialEq)]
pub struct HostApi {
    endpoints: HashMap<String, HostEndpoint>,
}

impl HostApi {
    pub fn builder() -> HostApiBuilder {
        HostApiBuilder::default()
    }

    pub fn endpoint(&self, name: &str) -> Option<&HostEndpoint> {
        self.endpoints.get(name)
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
    fn add_endpoint(
        mut self,
        name: &str,
        endpoint: HostEndpoint,
    ) -> HostApiBuilder {
        if self.endpoints.contains_key(name) {
            panic!("endpoint {} is already defined", name);
        }
        self.endpoints.insert(name.into(), endpoint);
        self
    }

    pub fn add_function_endpoint(
        self,
        name: &str,
        function: FunctionEndpoint,
    ) -> HostApiBuilder {
        self.add_endpoint(name, HostEndpoint::Function(function))
    }

    pub fn add_constant_endpoint(
        self,
        name: &str,
        constant: ConstantEndpoint,
    ) -> HostApiBuilder {
        self.add_endpoint(name, HostEndpoint::Constant(constant))
    }

    pub fn build(self) -> HostApi {
        HostApi {
            endpoints: self.endpoints,
        }
    }
}
