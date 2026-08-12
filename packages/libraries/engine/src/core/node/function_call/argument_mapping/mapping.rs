use std::collections::HashMap;

use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::{DowncastNode, Node};
use trace::{Tracable, TracableMut};

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
    ) -> Result<&ArgumentMappingEntry, Error> {
        self.map
            .get(name)
            .ok_or(Error::runtime_error("argument not found"))
    }

    pub fn get_node(&self, name: &str) -> Result<Node, Error> {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(node.clone()),
            _ => Err(Error::runtime_error("invalid parameter class")),
        }
    }

    pub fn required<T>(&self, name: &str) -> Result<&T, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(node
                .downcast::<T>()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            ArgumentMappingEntry::None => {
                Err(Error::missing_parameter("required argument missing"))
            }
            ArgumentMappingEntry::ValueList(nodes) => {
                Err(Error::too_many_parameters(nodes.len()))
            }
        }
    }

    pub fn optional<T>(&self, name: &str) -> Result<Option<&T>, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(Some(
                node.downcast::<T>()
                    .map_to_error_kind(ErrorKind::UnexpectedNodeType)?,
            )),
            ArgumentMappingEntry::None => Ok(None),
            ArgumentMappingEntry::ValueList(nodes) => {
                Err(Error::too_many_parameters(nodes.len()))
            }
        }
    }

    pub fn multiple<T>(&self, name: &str) -> Result<Vec<&T>, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::ValueList(nodes) => nodes
                .iter()
                .map(|node| -> Result<&T, Error> {
                    Ok(node
                        .downcast::<T>()
                        .map_to_error_kind(ErrorKind::UnexpectedNodeType)?)
                })
                .collect(),
            ArgumentMappingEntry::Value(node) => {
                Err(Error::runtime_error("argument is not a list")
                    .with_optional_span(node.hull()))
            }
            ArgumentMappingEntry::None => {
                Err(Error::runtime_error("argument list missing"))
            }
        }
    }

    pub fn optional_with_default<'a, T>(
        &'a self,
        name: &'a str,
        default: &'a Node,
    ) -> Result<&'a T, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            ArgumentMappingEntry::Value(node) => Ok(node
                .downcast::<T>()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            ArgumentMappingEntry::None => Ok(default
                .downcast::<T>()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            ArgumentMappingEntry::ValueList(nodes) => {
                Err(Error::too_many_parameters(nodes.len()))
            }
        }
    }
}
