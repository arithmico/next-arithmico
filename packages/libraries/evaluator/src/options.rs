mod stack;

use common::AngleUnit;
use node::{HostFunction, Node};
pub use stack::*;

use crate::{Api, Endpoint};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Options<'a> {
    pub stack: &'a Stack,
    pub api: &'a Api,
    pub angle_unit: AngleUnit,
}

impl<'a> Options<'a> {
    pub fn new(stack: &'a Stack, api: &'a Api, angle_unit: AngleUnit) -> Self {
        Options {
            stack,
            api,
            angle_unit,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Node> {
        if let Some(node) = self.stack.lookup(name) {
            return Some(node);
        }
        if let Some(endpoint) = self.api.endpoint(name) {
            return match endpoint {
                Endpoint::Function { .. } => {
                    Some(HostFunction::new(name).into())
                }
                Endpoint::Constant { executor, .. } => Some(executor(*self)),
            };
        }
        None
    }

    pub fn endpoint(&self, name: &str) -> Option<&Endpoint> {
        self.api.endpoint(name)
    }
}
