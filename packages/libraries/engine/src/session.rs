use crate::{api::load_host_api, Documentation};
use ast::{
    parse, serialize_node, EvaluateNodeContext, EvaluateNodeOptions, HostApi,
    Node, ParseNodeOptions, SerializeNodeOptions, Stack,
};
use evaluator::evaluate_node;
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
        options: &EvaluateNodeOptions,
    ) -> EvaluateNodeContext {
        EvaluateNodeContext::new(
            self.stack.clone(),
            options.clone(),
            self.host_api.clone(),
        )
    }

    pub fn push(&mut self, input: &str, options: &EvaluateNodeOptions) {
        let parse_node_options =
            ParseNodeOptions::new(options.get_decimal_format());
        let output: Result<String, SessionError> =
            parse(input, parse_node_options)
                .map_err(|error| SessionError::from(error))
                .and_then(|node| {
                    evaluate_node(&node, &self.create_context(options))
                        .map_err(|error| SessionError::from(error))
                })
                .and_then(|node: Node| {
                    if let Node::Definition(definition) = &node {
                        self.stack.insert(
                            &definition.symbol,
                            *definition.expression.clone(),
                        );
                    }

                    serialize_node(
                        &node,
                        &SerializeNodeOptions::new(
                            options.get_decimal_places().into(),
                            options.get_decimal_format(),
                        ),
                    )
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
}
