use crate::{Documentation, api::load_host_api};
use std::collections::HashMap;
use std::sync::Arc;

use common::AngleUnit;
pub use entry::SessionEntry;
pub use error::*;
use evaluator::{Api, EvaluateNode, Options, Stack};
use node::Node;
use parser::parse;
use translate::Language;

mod entry;
mod error;

#[derive(Debug, Clone)]
pub struct Session {
    stack: Stack,
    host_api: Arc<Api>,
    entries: Vec<SessionEntry>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            host_api: Arc::new(load_host_api()),
            entries: Vec::new(),
        }
    }

    pub fn create_options<'a>(&'a self, angle_unit: AngleUnit) -> Options<'a> {
        Options::new(&self.stack, self.host_api.as_ref(), angle_unit)
    }

    fn evaluate_input(
        &mut self,
        input: &str,
        language: Language,
        angle_unit: AngleUnit,
    ) -> Result<Node, SessionError> {
        let options = self.create_options(angle_unit);

        let node = parse(input, language)?;

        let evaluatd_node = node.evaluate(options)?;

        let output = if let Node::Definition(node) = &evaluatd_node {
            self.stack.insert(&node.symbol, *node.expression.clone());
            *node.expression.clone()
        } else {
            evaluatd_node
        };

        Ok(output)
    }

    pub fn push(
        &mut self,
        input: &str,
        language: Language,
        angle_unit: AngleUnit,
    ) {
        let output = self.evaluate_input(input, language, angle_unit);

        self.entries.push(SessionEntry {
            input: input.to_string(),
            output,
        });
    }

    pub fn entries(&self) -> &[SessionEntry] {
        &self.entries
    }

    pub fn last_entry(&self) -> Option<&SessionEntry> {
        self.entries.last()
    }

    pub fn documentation(&self) -> Documentation {
        Documentation::from(&(*self.host_api))
    }

    pub fn stack_entries(&self) -> HashMap<String, Node> {
        self.stack.entries()
    }
}
