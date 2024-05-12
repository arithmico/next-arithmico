use std::{collections::HashMap, rc::Rc};

use crate::{context::Context, node::Node, Language};

use super::host_endpoint::{ConstantEndpoint, FunctionEndpoint, HostEndpoint};

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
                    .serialize(&Context::new(Rc::new(self.clone()))),
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
        executor: FunctionEndpoint,
        arguments: Vec<String>,
        documentation: HashMap<Language, String>,
    ) -> HostApiBuilder {
        self.add_endpoint(
            name,
            HostEndpoint::Function {
                executor,
                arguments,
                description: documentation,
            },
        )
    }

    pub fn add_constant_endpoint(
        self,
        name: &str,
        executor: ConstantEndpoint,
        documentation: HashMap<Language, String>,
    ) -> HostApiBuilder {
        self.add_endpoint(
            name,
            HostEndpoint::Constant {
                executor,
                description: documentation,
            },
        )
    }

    pub fn build(self) -> HostApi {
        HostApi {
            endpoints: self.endpoints,
        }
    }
}
