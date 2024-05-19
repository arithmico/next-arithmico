use std::{collections::HashMap, rc::Rc};

use crate::{
    context::{Context, Settings, Stack},
    node::Node,
    Language,
};

use super::{host_api_module::HostApiModule, host_endpoint::HostEndpoint};

pub struct Documentation {
    pub synopsis: String,
    pub description: String,
}

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

    pub fn get_documentation(&self, language: &Language) -> Vec<Documentation> {
        self.endpoints
            .iter()
            .map(|(name, endpoint)| match endpoint {
                HostEndpoint::Function {
                    arguments,
                    description,
                    ..
                } => Documentation {
                    synopsis: Node::FunctionCall {
                        target: Node::Symbol { name: name.clone() }.into(),
                        arguments: arguments
                            .iter()
                            .map(|argument| Node::Symbol {
                                name: argument.clone(),
                            })
                            .collect(),
                    }
                    .serialize(&Context::new(
                        Stack::new(),
                        Settings::default(),
                        Rc::new(self.clone()),
                    )),
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
        module_loader: fn() -> HostApiModule,
    ) -> HostApiBuilder {
        let module = module_loader();
        for (name, endpoint) in module.get_endpoints().iter() {
            if self.endpoints.contains_key(name) {
                panic!("endpoint \"{}\" already exists", name);
            }
            self.endpoints.insert(name.clone(), endpoint.clone());
        }
        self
    }

    pub fn build(self) -> HostApi {
        HostApi {
            endpoints: self.endpoints,
        }
    }
}
