use crate::core::{
    evaluate_node, parse, serialize_node, Context, HostApi, Node, Stack,
};
use crate::{api::load_host_api, Documentation};
use crate::{DecimalFormat, DecimalPlaces};
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

    fn create_context(
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

    pub fn push(
        &mut self,
        input: &str,
        decimal_places: DecimalPlaces,
        decimal_format: DecimalFormat,
    ) {
        let context = self.create_context(decimal_places, decimal_format);

        let output: Result<String, SessionError> = parse(input, &context)
            .map_err(|error| SessionError::from(error))
            .and_then(|node| {
                evaluate_node(&node, &context)
                    .map_err(|error| SessionError::from(error))
            })
            .and_then(|node: Node| {
                if let Node::Definition(definition) = &node {
                    self.stack.insert(
                        &definition.symbol,
                        *definition.expression.clone(),
                    );
                }

                serialize_node(&node, &context)
                    .map_err(|error| SessionError::from(error))
            });

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
