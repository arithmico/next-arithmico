use std::collections::HashMap;

use crate::core::{EvaluateNodeError, Node, NodeCast};

use super::entry::ArgumentBindingEntry;

#[derive(Debug, Clone)]
pub struct ArgumentsBinding {
    map: HashMap<String, ArgumentBindingEntry>,
}

impl ArgumentsBinding {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert_value<T: ToString>(&mut self, name: T, node: Node) {
        self.map
            .insert(name.to_string(), ArgumentBindingEntry::Value(node));
    }

    pub fn insert_value_list<T: ToString>(
        &mut self,
        name: T,
        nodes: Vec<Node>,
    ) {
        self.map
            .insert(name.to_string(), ArgumentBindingEntry::ValueList(nodes));
    }

    pub fn insert_none<T: ToString>(&mut self, name: T) {
        self.map
            .insert(name.to_string(), ArgumentBindingEntry::None);
    }

    pub fn parameter_names(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    fn get_parameter_entry(
        &self,
        name: &str,
    ) -> Result<&ArgumentBindingEntry, EvaluateNodeError> {
        self.map
            .get(name)
            .ok_or(EvaluateNodeError::runtime_error("argument not found"))
    }

    pub fn get_node(&self, name: &str) -> Result<Node, EvaluateNodeError> {
        match self.get_parameter_entry(name)? {
            ArgumentBindingEntry::Value(node) => Ok(node.clone()),
            _ => {
                Err(EvaluateNodeError::runtime_error("invalid parameter class"))
            }
        }
    }

    pub fn required<T>(&self, name: &str) -> Result<T, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentBindingEntry::Value(node) => T::try_from_node(node),
            ArgumentBindingEntry::None => {
                Err(EvaluateNodeError::missing_parameter(
                    "required argument missing",
                ))
            }
            ArgumentBindingEntry::ValueList(nodes) => {
                Err(EvaluateNodeError::too_many_parameters(nodes.len()))
            }
        }
    }

    pub fn optional<T>(
        &self,
        name: &str,
    ) -> Result<Option<T>, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentBindingEntry::Value(node) => {
                Ok(Some(T::try_from_node(node)?))
            }
            ArgumentBindingEntry::None => Ok(None),
            ArgumentBindingEntry::ValueList(nodes) => {
                Err(EvaluateNodeError::too_many_parameters(nodes.len()))
            }
        }
    }

    pub fn multiple<T>(&self, name: &str) -> Result<Vec<T>, EvaluateNodeError>
    where
        T: NodeCast,
    {
        match self.get_parameter_entry(name)? {
            ArgumentBindingEntry::ValueList(nodes) => {
                nodes.iter().map(T::try_from_node).collect()
            }
            ArgumentBindingEntry::Value(node) => {
                Err(EvaluateNodeError::runtime_error("argument is not a list")
                    .with_tracable(node))
            }
            ArgumentBindingEntry::None => {
                Err(EvaluateNodeError::runtime_error("argument list missing"))
            }
        }
    }
}
