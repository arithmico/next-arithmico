use std::collections::HashMap;

mod entry;

use node::{DowncastNode, Node};
use trace::{Tracable, TracableMut};

use crate::{
    Error, ErrorKind, MapToEvaluatorError, argument_mapping::entry::Entry,
};

#[derive(Debug, Clone)]
pub struct ArgumentMapping {
    map: HashMap<String, Entry>,
}

impl ArgumentMapping {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert_value<T: ToString>(&mut self, name: T, node: Node) {
        self.map.insert(name.to_string(), Entry::Value(node));
    }

    pub fn insert_value_list<T: ToString>(
        &mut self,
        name: T,
        nodes: Vec<Node>,
    ) {
        self.map.insert(name.to_string(), Entry::List(nodes));
    }

    pub fn insert_none<T: ToString>(&mut self, name: T) {
        self.map.insert(name.to_string(), Entry::None);
    }

    pub fn parameter_names(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    fn get_parameter_entry(&self, name: &str) -> Result<&Entry, Error> {
        self.map
            .get(name)
            .ok_or(Error::runtime_error("argument not found"))
    }

    pub fn get_node(&self, name: &str) -> Result<Node, Error> {
        match self.get_parameter_entry(name)? {
            Entry::Value(node) => Ok(node.clone()),
            _ => Err(Error::runtime_error("invalid parameter class")),
        }
    }

    pub fn required<T>(&self, name: &str) -> Result<&T, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            Entry::Value(node) => Ok(node
                .downcast::<T>()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            Entry::None => {
                Err(Error::missing_parameter("required argument missing"))
            }
            Entry::List(nodes) => Err(Error::too_many_parameters(nodes.len())),
        }
    }

    pub fn optional<T>(&self, name: &str) -> Result<Option<&T>, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            Entry::Value(node) => Ok(Some(
                node.downcast::<T>()
                    .map_to_error_kind(ErrorKind::UnexpectedNodeType)?,
            )),
            Entry::None => Ok(None),
            Entry::List(nodes) => Err(Error::too_many_parameters(nodes.len())),
        }
    }

    pub fn multiple<T>(&self, name: &str) -> Result<Vec<&T>, Error>
    where
        T: DowncastNode,
    {
        match self.get_parameter_entry(name)? {
            Entry::List(nodes) => nodes
                .iter()
                .map(|node| -> Result<&T, Error> {
                    Ok(node
                        .downcast::<T>()
                        .map_to_error_kind(ErrorKind::UnexpectedNodeType)?)
                })
                .collect(),
            Entry::Value(node) => {
                Err(Error::runtime_error("argument is not a list")
                    .with_optional_span(node.hull()))
            }
            Entry::None => Err(Error::runtime_error("argument list missing")),
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
            Entry::Value(node) => Ok(node
                .downcast::<T>()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            Entry::None => Ok(default
                .downcast::<T>()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            Entry::List(nodes) => Err(Error::too_many_parameters(nodes.len())),
        }
    }
}
