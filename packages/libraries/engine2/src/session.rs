use ast::Node;
use entry::SessionEntry;
use error::SessionError;
use evaluator::{evaluate_node, EvaluateNodeContext};
use parser::parse;
use serializer::{serialize_node, SerializeNodeOptions};

mod entry;
mod error;

pub struct Session {
    context: EvaluateNodeContext,
    entries: Vec<SessionEntry>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            context: EvaluateNodeContext::default(),
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, input: &str) {
        let output: Result<String, SessionError> = parse(input)
            .map_err(|error| SessionError::from(error))
            .and_then(|node| {
                evaluate_node(&node, &self.context)
                    .map_err(|error| SessionError::from(error))
            })
            .and_then(|node: Node| {
                if let Node::Definition(definition) = &node {
                    self.context.stack.insert(
                        &definition.symbol,
                        *definition.expression.clone(),
                    );
                }

                serialize_node(
                    &node,
                    &SerializeNodeOptions::new(
                        self.context.options.get_decimal_places().into(),
                        self.context.options.get_decimal_format(),
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
}
