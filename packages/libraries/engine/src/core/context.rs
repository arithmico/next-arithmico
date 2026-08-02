use std::sync::Arc;

use node::{HostFunction, Node};

mod decimal_places;

pub use decimal_places::*;
use translate::Language;

use super::{HostApi, HostEndpoint, Stack};

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub stack: Stack,
    pub decimal_places: DecimalPlaces,
    pub language: Language,
    pub host_api: Arc<HostApi>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: Stack::new(),
            decimal_places: DecimalPlaces::default(),
            language: Language::default(),
            host_api: HostApi::empty().into(),
        }
    }
}

impl Context {
    pub fn new(
        stack: Stack,
        decimal_places: DecimalPlaces,
        language: Language,
        host_api: Arc<HostApi>,
    ) -> Context {
        Context {
            stack,
            language,
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
