use std::sync::Arc;

use node::{HostFunction, Node};

use serde::{Deserialize, Serialize};
use serializer::DecimalPlaces;
use translate::Language;

use super::{HostApi, HostEndpoint, Stack};

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub stack: Stack,
    pub decimal_places: DecimalPlaces,
    pub angle_unit: AngleUnit,
    pub language: Language,
    pub host_api: Arc<HostApi>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AngleUnit {
    Radian,
    Degree,
}

impl Default for AngleUnit {
    fn default() -> Self {
        Self::Radian
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: Stack::new(),
            decimal_places: DecimalPlaces::default(),
            angle_unit: Default::default(),
            language: Language::default(),
            host_api: HostApi::empty().into(),
        }
    }
}

impl Context {
    pub fn new(
        stack: Stack,
        decimal_places: DecimalPlaces,
        angle_unit: AngleUnit,
        language: Language,
        host_api: Arc<HostApi>,
    ) -> Context {
        Context {
            stack,
            language,
            angle_unit,
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
