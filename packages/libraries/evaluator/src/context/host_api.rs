use std::collections::HashMap;

use ast::{FunctionCall, Symbol};
use common::{DecimalFormat, DecimalPlaces, Language};
use log::info;
use serializer::{serialize_node, SerializeNodeOptions};

pub use endpoint::HostEndpoint;
pub use module::HostApiModule;

mod endpoint;
mod module;

pub struct Documentation {
    pub synopsis: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HostApi {
    endpoints: HashMap<String, HostEndpoint>,
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

    pub fn get_documentation(&self, language: &Language) -> Vec<Documentation> {
        self.endpoints
            .iter()
            .map(|(name, endpoint)| match endpoint {
                HostEndpoint::Function {
                    arguments,
                    description,
                    ..
                } => Documentation {
                    synopsis: serialize_node(
                        &FunctionCall::new(
                            Symbol::new(name),
                            arguments
                                .iter()
                                .map(|argument| Symbol::new(argument).into())
                                .collect(),
                        ),
                        &SerializeNodeOptions::new(
                            DecimalPlaces::from(5),
                            DecimalFormat::from(language),
                        ),
                    )
                    .unwrap(),
                    description: description
                        .get(language)
                        .and_then(|description| Some(description.clone()))
                        .unwrap_or_else(|| {
                            String::from("No documentation available")
                        }),
                },
                HostEndpoint::Constant { description, .. } => Documentation {
                    synopsis: name.clone(),
                    description: description
                        .get(language)
                        .and_then(|description| Some(description.clone()))
                        .unwrap_or_else(|| {
                            String::from("No documentation available")
                        }),
                },
            })
            .collect()
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
            for (name, endpoint) in module.get_endpoints().iter() {
                if self.endpoints.contains_key(name) {
                    panic!("endpoint \"{}\" already exists", name);
                }
                self.endpoints.insert(name.clone(), endpoint.clone());
            }
        } else {
            info!("skip module");
        }
        self
    }

    pub fn build(self) -> HostApi {
        HostApi {
            endpoints: self.endpoints,
        }
    }
}
