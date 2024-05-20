use std::{collections::HashMap, rc::Rc};

use crate::{core::node::Node, load_host_api};

use super::{host_api::HostApi, settings::Settings, HostEndpoint};

pub type Stackframe = HashMap<String, Node>;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    frames: Vec<Stackframe>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            frames: vec![HashMap::new()],
        }
    }

    pub fn get_frames(&self) -> &Vec<Stackframe> {
        &self.frames
    }

    pub fn add_frame(&mut self) {
        self.frames.push(HashMap::new());
    }

    pub fn insert(&mut self, name: &str, node: Node) {
        if self.frames.is_empty() {
            self.frames.push(HashMap::new());
        }
        self.frames.last_mut().unwrap().insert(name.into(), node);
    }

    pub fn lookup(&self, name: &str) -> Option<Node> {
        for frame in &self.frames {
            let node_option = frame.get(name);
            if let Some(node) = node_option {
                return Some(node.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub stack: Stack,
    pub settings: Settings,
    pub host_api: Rc<HostApi>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: Stack::new(),
            settings: Settings::default(),
            host_api: load_host_api().into(),
        }
    }
}

impl Context {
    pub fn new(
        stack: Stack,
        settings: Settings,
        host_api: Rc<HostApi>,
    ) -> Context {
        Context {
            stack,
            settings,
            host_api,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Node> {
        if let Some(node) = self.stack.lookup(name) {
            return Some(node);
        }
        if let Some(endpoint) = self.host_api.endpoint(name) {
            return match endpoint {
                HostEndpoint::Function { .. } => {
                    Some(Node::HostApiFunctionEndpoint {
                        name: name.to_string(),
                    })
                }
                HostEndpoint::Constant { executor, .. } => Some(executor(self)),
            };
        }
        None
    }

    pub fn endpoint(&self, name: &str) -> Option<&HostEndpoint> {
        self.host_api.endpoint(name)
    }
}
