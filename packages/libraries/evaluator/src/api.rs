mod endpoint;
mod endpoint_metadata;
mod module;

use std::collections::BTreeMap;

pub use endpoint::*;
pub use endpoint_metadata::*;
pub use module::*;

#[derive(Debug)]
pub struct Api {
    endpoints: BTreeMap<String, Endpoint>,
}

impl PartialEq for Api {
    fn eq(&self, other: &Self) -> bool {
        self.endpoints.keys().collect::<Vec<_>>()
            == other.endpoints.keys().collect::<Vec<_>>()
    }
}

impl Api {
    pub fn empty() -> Self {
        Self {
            endpoints: BTreeMap::new(),
        }
    }

    pub fn builder() -> ApiBuilder {
        ApiBuilder::default()
    }

    pub fn endpoint(&self, name: &str) -> Option<&Endpoint> {
        self.endpoints.get(name)
    }

    pub fn endpoints(&self) -> &BTreeMap<String, Endpoint> {
        &self.endpoints
    }
}

impl Default for Api {
    fn default() -> Self {
        Self {
            endpoints: Default::default(),
        }
    }
}

pub struct ApiBuilder {
    endpoints: BTreeMap<String, Endpoint>,
}

impl Default for ApiBuilder {
    fn default() -> Self {
        Self {
            endpoints: Default::default(),
        }
    }
}

impl ApiBuilder {
    pub fn module(
        mut self,
        feature_flag: bool,
        module_loader: fn() -> ApiModule,
    ) -> ApiBuilder {
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

    pub fn build(self) -> Api {
        Api {
            endpoints: self.endpoints,
        }
    }
}
