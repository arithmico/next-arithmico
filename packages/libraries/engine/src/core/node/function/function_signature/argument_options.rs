use std::{collections::{HashMap, HashSet}};

use translate::{Language, Translatable, use_translate};

use crate::{
    Context, SerializeNode, core::{Node, NodeType}
};

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentOptions {
    preprocess: Preprocess,
    cardinality: Cardinality,
    node_types: HashSet<NodeType>,
}

impl ArgumentOptions {
    pub fn new() -> Self {
        Self {
            preprocess: Preprocess::None,
            cardinality: Cardinality::Required,
            node_types: HashSet::new(),
        }
    }

    pub fn preprocess(&self) -> Preprocess {
        self.preprocess.clone()
    }

    pub fn set_preprocess(&mut self, preprocess: Preprocess) {
        self.preprocess = preprocess;
    }

    pub fn cardinality(&self) -> Cardinality {
        self.cardinality.clone()
    }

    pub fn set_cardinality(&mut self, cardinality: Cardinality) {
        self.cardinality = cardinality;
    }

    pub fn node_types(&self) -> HashSet<NodeType> {
        self.node_types.clone()
    }

    pub fn add_node_type(&mut self, node_type: NodeType) {
        self.node_types.insert(node_type);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Preprocess {
    None,
    Evaluate,
    // TODO: Reduce
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cardinality {
    Required,
    Optional,
    OptionalWithDefault { default: Node },
    Multiple { min: usize, max: Option<usize> },
}

impl Cardinality {
    fn to_translation_string_with_keys(&self) -> (&'static str, Option<HashMap<String, String>>) {
        match self {
            Cardinality::Required => ("engine.cardinality.required", None),
            Cardinality::Optional => ("engine.cardinality.optional", None),
            Cardinality::OptionalWithDefault { default } => {
                let mut keys = HashMap::new();
                keys.insert("default".to_string(), format!("{default:?}"));

                ("engine.cardinality.default", Some(keys))
            }
            Cardinality::Multiple { min, max } => {
                let mut keys = HashMap::new();
                keys.insert("min".to_string(), min.to_string());

                match max {
                    Some(max) => {
                        keys.insert("max".to_string(), max.to_string());
                        ("engine.cardinality.range", Some(keys))
                    }
                    None => ("engine.cardinality.range.open", Some(keys)),
                }
            }
        }
    }

    fn translation_key_and_args_with_context(
        &self,
        context: &Context,
    ) -> (&'static str, Option<HashMap<String, String>>) {
        match self {
            Cardinality::Required => ("engine.cardinality.required", None),
            Cardinality::Optional => ("engine.cardinality.optional", None),
            Cardinality::OptionalWithDefault { default } => {
                let mut keys = HashMap::new();

                let default_str = default
                    .serialize(context)
                    .unwrap_or_else(|_| format!("{default:?}"));

                keys.insert("default".to_string(), default_str);

                ("engine.cardinality.default", Some(keys))
            }

            Cardinality::Multiple { min, max } => {
                let mut keys = HashMap::new();
                keys.insert("min".to_string(), min.to_string());

                match max {
                    Some(max) => {
                        keys.insert("max".to_string(), max.to_string());
                        ("engine.cardinality.range", Some(keys))
                    }
                    None => ("engine.cardinality.range.open", Some(keys)),
                }
            }
        }
    }

    pub fn translate_with_context(
        &self,
        _language: &Language,
        context: &Context,
    ) -> Result<String, translate::TranslationError> {
        let translate = use_translate();
        let (id, args) = self.translation_key_and_args_with_context(context);

        translate(id, args)
            .map_err(|err| translate::TranslationError::MissingKey(err.to_string()))
    }
}

impl Translatable for Cardinality {
    fn translate(
        &self,
        _language: Language,
    ) -> Result<String, translate::TranslationError> {
        let translate = use_translate();
        let (id, keys) = self.to_translation_string_with_keys();

        translate(id, keys)
        .map_err(|e| translate::TranslationError::MissingKey(e.to_string()))
    }
}