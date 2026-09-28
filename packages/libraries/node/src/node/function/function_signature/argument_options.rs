use std::collections::HashSet;

use crate::{Node, NodeType};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ArgumentOptions {
    preprocess: Preprocess,
    cardinality: Cardinality,
    node_types: HashSet<NodeType>,
}

impl ArgumentOptions {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Preprocess {
    #[default]
    None,
    Evaluate,
    // TODO: Reduce
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Cardinality {
    #[default]
    Required,
    Optional,
    OptionalWithDefault {
        default: Node,
    },
    Multiple {
        min: usize,
        max: Option<usize>,
    },
}
