use std::collections::HashMap;

use ast::Node;

use crate::EvaluateNodeError;

use super::entry::ArgumentMappingEntry;

#[derive(Debug, Clone)]
pub struct ArgumentMapping {
    map: HashMap<String, ArgumentMappingEntry>,
}

impl ArgumentMapping {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert_value<T: ToString>(&mut self, name: T, node: Node) {
        self.map
            .insert(name.to_string(), ArgumentMappingEntry::Value(node));
    }

    pub fn insert_value_list<T: ToString>(
        &mut self,
        name: T,
        nodes: Vec<Node>,
    ) {
        self.map
            .insert(name.to_string(), ArgumentMappingEntry::ValueList(nodes));
    }

    pub fn insert_none<T: ToString>(&mut self, name: T) {
        self.map
            .insert(name.to_string(), ArgumentMappingEntry::None);
    }

    pub fn parameter_names(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    fn get_parameter_entry(
        &self,
        name: &str,
    ) -> Result<&ArgumentMappingEntry, EvaluateNodeError> {
        self.map
            .get(name)
            .ok_or(EvaluateNodeError::runtime_error("argument not found"))
    }

    pub fn get_parameter_value(
        &self,
        name: &str,
    ) -> Result<Node, EvaluateNodeError> {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(node.clone()),
            _ => {
                Err(EvaluateNodeError::runtime_error("invalid parameter class"))
            }
        }
    }
}
