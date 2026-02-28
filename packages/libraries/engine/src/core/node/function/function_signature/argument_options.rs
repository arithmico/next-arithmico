use std::collections::{HashMap, HashSet};

use translate::use_translate;

use crate::{
    core::{Node, NodeType},
    Context, Serialize,
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

impl Serialize for Cardinality {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, crate::SerializeNodeError> {
        let translate = use_translate();
        let (id, keys) = match self {
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
        };

        translate(id, keys).map_err(|_| crate::SerializeNodeError::InvalidNode)
    }
}
