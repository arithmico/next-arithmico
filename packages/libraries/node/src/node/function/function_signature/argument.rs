use translate_core::{Language, RenderedTranslatedMessage};

use crate::{Node, NodeType};

use super::argument_options::{ArgumentOptions, Cardinality, Preprocess};

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    name: String,
    description: RenderedTranslatedMessage,
    options: ArgumentOptions,
}

#[allow(dead_code)]
impl Argument {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: RenderedTranslatedMessage::default(),
            options: ArgumentOptions::default(),
        }
    }

    pub fn optional(mut self) -> Self {
        self.options.set_cardinality(Cardinality::Optional);
        self
    }

    pub fn repeatable(
        mut self,
        min: impl Into<usize>,
        max: impl Into<Option<usize>>,
    ) -> Self {
        self.options.set_cardinality(Cardinality::Multiple {
            min: min.into(),
            max: max.into(),
        });
        self
    }

    pub fn min_repetitions(mut self, min: usize) -> Self {
        match self.options.cardinality() {
            Cardinality::Multiple { min: _, max } => {
                self.options
                    .set_cardinality(Cardinality::Multiple { min, max });
            }
            _ => {
                self.options
                    .set_cardinality(Cardinality::Multiple { min, max: None });
            }
        }
        self
    }

    pub fn max_repetitions(mut self, max: usize) -> Self {
        match self.options.cardinality() {
            Cardinality::Multiple { min, max: _ } => {
                self.options.set_cardinality(Cardinality::Multiple {
                    min,
                    max: Some(max),
                });
            }
            _ => {
                self.options.set_cardinality(Cardinality::Multiple {
                    min: 1,
                    max: None,
                });
            }
        }
        self
    }

    pub fn evaluate(mut self) -> Self {
        self.options.set_preprocess(Preprocess::Evaluate);
        self
    }

    pub fn default(mut self, node: Node) -> Self {
        self.options
            .set_cardinality(Cardinality::OptionalWithDefault {
                default: node,
            });
        self
    }

    pub fn node_type(mut self, node_type: NodeType) -> Self {
        self.options.add_node_type(node_type);
        self
    }

    pub fn description<T: ToString>(
        mut self,
        language: Language,
        description: T,
    ) -> Self {
        self.description
            .add_message(language, Ok(description.to_string()));
        self
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> RenderedTranslatedMessage {
        self.description.clone()
    }

    pub fn get_options(&self) -> &ArgumentOptions {
        &self.options
    }

    pub fn has_node_type(&self, node_type: NodeType) -> bool {
        self.get_options().node_types().contains(&node_type)
            || self.get_options().node_types().contains(&NodeType::Any)
    }
}
