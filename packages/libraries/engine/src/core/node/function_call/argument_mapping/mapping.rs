use std::collections::HashMap;

use node::Node;

use crate::core::{EvaluateNodeError, NodeCast};

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

    pub fn get_node(&self, name: &str) -> Result<Node, EvaluateNodeError> {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(node.clone()),
            _ => {
                Err(EvaluateNodeError::runtime_error("invalid parameter class"))
            }
        }
    }

    pub fn required<T>(&self, name: &str) -> Result<&T, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => T::downcast(node),
            ArgumentMappingEntry::None => {
                Err(EvaluateNodeError::missing_parameter(
                    "required argument missing",
                ))
            }
            ArgumentMappingEntry::ValueList(nodes) => {
                Err(EvaluateNodeError::too_many_parameters(nodes.len()))
            }
        }
    }

    pub fn optional<T>(
        &self,
        name: &str,
    ) -> Result<Option<&T>, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(Some(T::downcast(node)?)),
            ArgumentMappingEntry::None => Ok(None),
            ArgumentMappingEntry::ValueList(nodes) => {
                Err(EvaluateNodeError::too_many_parameters(nodes.len()))
            }
        }
    }

    pub fn multiple<T>(&self, name: &str) -> Result<Vec<&T>, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::ValueList(nodes) => {
                nodes.iter().map(T::downcast).collect()
            }
            ArgumentMappingEntry::Value(node) => {
                Err(EvaluateNodeError::runtime_error("argument is not a list")
                    .with_tracable(node))
            }
            ArgumentMappingEntry::None => {
                Err(EvaluateNodeError::runtime_error("argument list missing"))
            }
        }
    }

    pub fn optional_with_default<'a, T>(
        &'a self,
        name: &'a str,
        default: &'a Node,
    ) -> Result<&'a T, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => T::downcast(node),
            ArgumentMappingEntry::None => T::downcast(default),
            ArgumentMappingEntry::ValueList(nodes) => {
                Err(EvaluateNodeError::too_many_parameters(nodes.len()))
            }
        }
    }
}
