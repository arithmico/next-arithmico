use std::collections::{HashMap, HashSet};

use translate::use_translate;

use crate::{
    core::{Context, Node, NodeType},
    SerializeNode,
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
    pub fn to_translated_string(
        &self,
        context: &Context,
    ) -> Result<String, &'static str> {
        let translate = use_translate();

        match self {
            Cardinality::Required => translate("engine.cardinality.required", None),
            Cardinality::Optional => translate("engine.cardinality.optional", None),
            Cardinality::OptionalWithDefault { default } => {
                let mut keys = HashMap::new();
                keys.insert(
                    "default".to_string(),
                    default.serialize(context).unwrap(),
                );

                translate("engine.cardinality.default", Some(keys))
            }
            Cardinality::Multiple { min, max } => {
                let mut keys = HashMap::new();
                keys.insert("min".to_string(), min.to_string());

                match max {
                    Some(max) => {
                        keys.insert("max".to_string(), max.to_string());
                        translate("engine.cardinality.range", Some(keys))
                    }
                    None => translate("engine.cardinality.range.open", Some(keys)),
                }
            }
        }
    }
}
