use translate_core::{Language, TranslationTemplateCollection};

use crate::{Node, NodeType};

use super::argument_options::{ArgumentOptions, Cardinality, Preprocess};

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    name: String,
    description: TranslationTemplateCollection,
    options: ArgumentOptions,
}

#[allow(dead_code)]
impl Argument {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: TranslationTemplateCollection::new(),
            options: ArgumentOptions::new(),
        }
    }

    pub fn optional(mut self) -> Self {
        self.options.set_cardinality(Cardinality::Optional);
        self
    }

    pub fn repeatable(mut self) -> Self {
        self.options
            .set_cardinality(Cardinality::Multiple { min: 1, max: None });
        self
    }

    pub fn min_repetitions(mut self, min: usize) -> Self {
        if let Cardinality::Multiple { min: _, max } =
            self.options.cardinality()
        {
            self.options
                .set_cardinality(Cardinality::Multiple { min, max });
        } else {
            self.options
                .set_cardinality(Cardinality::Multiple { min, max: None });
        }
        self
    }

    pub fn max_repetitions(mut self, max: usize) -> Self {
        if let Cardinality::Multiple { min, max: _ } =
            self.options.cardinality()
        {
            self.options.set_cardinality(Cardinality::Multiple {
                min,
                max: Some(max),
            });
        } else {
            self.options
                .set_cardinality(Cardinality::Multiple { min: 1, max: None });
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
            .add_translation(language, description.to_string());
        self
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn options(&self) -> &ArgumentOptions {
        &self.options
    }

    pub fn has_node_type(&self, node_type: NodeType) -> bool {
        self.options().node_types().contains(&node_type)
            || self.options().node_types().contains(&NodeType::Any)
    }
}
