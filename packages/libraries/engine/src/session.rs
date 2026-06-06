use crate::core::{
    Context, HostApi, Node, SerializeUtils, Stack, evaluate_node, parse,
};
use crate::{DecimalFormat, DecimalPlaces};
use crate::{Documentation, api::load_host_api};
use std::collections::HashMap;
use std::sync::Arc;

pub use entry::SessionEntry;
pub use error::*;

mod entry;
mod error;

#[derive(Debug, Clone)]
pub struct Session {
    stack: Stack,
    host_api: Arc<HostApi>,
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

    pub fn create_context(
        &self,
        decimal_places: DecimalPlaces,
        decimal_format: DecimalFormat,
    ) -> Context {
        Context::new(
            self.stack.clone(),
            decimal_places,
            decimal_format,
            self.host_api.clone(),
        )
    }

    fn evaluate_input(
        &mut self,
        input: &str,
        decimal_places: DecimalPlaces,
        decimal_format: DecimalFormat,
    ) -> Result<Node, SessionError> {
        let context = self.create_context(decimal_places, decimal_format);

        let node = parse(input, &context)?;

        let evaluatd_node =
            evaluate_node(&node, &context)?.normalize_node(&context)?;

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
        decimal_places: DecimalPlaces,
        decimal_format: DecimalFormat,
    ) {
        let output = self.evaluate_input(input, decimal_places, decimal_format);

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
