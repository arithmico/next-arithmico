mod host_api;
mod options;
mod stack;

use ast::{HostFunction, Node};
use std::rc::Rc;

pub use host_api::*;
pub use options::*;
pub use stack::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluateNodeContext {
    pub stack: Stack,
    pub options: EvaluateNodeOptions,
    pub host_api: Rc<HostApi>,
}

impl Default for EvaluateNodeContext {
    fn default() -> Self {
        Self {
            stack: Stack::new(),
            options: EvaluateNodeOptions::default(),
            host_api: HostApi::empty().into(),
        }
    }
}

impl EvaluateNodeContext {
    pub fn new(
        stack: Stack,
        options: EvaluateNodeOptions,
        host_api: Rc<HostApi>,
    ) -> EvaluateNodeContext {
        EvaluateNodeContext {
            stack,
            options,
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
                    Some(HostFunction::new(name).into())
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
