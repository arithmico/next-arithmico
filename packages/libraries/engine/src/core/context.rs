use std::sync::Arc;

use crate::core::{HostFunction, Node};

mod decimal_format;
mod decimal_places;
mod host_api;
mod stack;

pub use decimal_format::*;
pub use decimal_places::*;
pub use host_api::*;
pub use stack::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluateNodeContext {
    pub stack: Stack,
    pub decimal_places: DecimalPlaces,
    pub decimal_format: DecimalFormat,
    pub host_api: Arc<HostApi>,
}

impl Default for EvaluateNodeContext {
    fn default() -> Self {
        Self {
            stack: Stack::new(),
            decimal_places: DecimalPlaces::default(),
            decimal_format: DecimalFormat::default(),
            host_api: HostApi::empty().into(),
        }
    }
}

impl EvaluateNodeContext {
    pub fn new(
        stack: Stack,
        decimal_places: DecimalPlaces,
        decimal_format: DecimalFormat,
        host_api: Arc<HostApi>,
    ) -> EvaluateNodeContext {
        EvaluateNodeContext {
            stack,
            decimal_format,
            decimal_places,
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
