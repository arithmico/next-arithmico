use std::collections::{HashMap, VecDeque};

mod entry;

use node::{
    Cardinality, DowncastNodeError, FunctionSignature, GetNodeType, Node,
    Preprocess,
};
use trace::{CombineHulls, Tracable, TracableMut};

use crate::{
    Error, ErrorKind, EvaluateNode, MapToEvaluatorError, Options,
    argument_mapping::entry::Entry,
};

#[derive(Debug, Clone, Default)]
pub struct ArgumentMapping {
    map: HashMap<String, Entry>,
}

impl ArgumentMapping {
    pub fn new() -> Self {
        Self::default()
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

    pub fn required<'a, T>(&'a self, name: &str) -> Result<&'a T, Error>
    where
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>,
    {
        match self.get_parameter_entry(name)? {
            Entry::Value(node) => Ok(node
                .try_into()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            Entry::None => {
                Err(Error::missing_parameter("required argument missing"))
            }
            Entry::List(nodes) => Err(Error::too_many_parameters(nodes.len())),
        }
    }

    pub fn optional<'a, T>(&'a self, name: &str) -> Result<Option<&'a T>, Error>
    where
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>,
    {
        match self.get_parameter_entry(name)? {
            Entry::Value(node) => Ok(Some(
                node.try_into()
                    .map_to_error_kind(ErrorKind::UnexpectedNodeType)?,
            )),
            Entry::None => Ok(None),
            Entry::List(nodes) => Err(Error::too_many_parameters(nodes.len())),
        }
    }

    pub fn multiple<'a, T>(&'a self, name: &str) -> Result<Vec<&'a T>, Error>
    where
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>,
    {
        match self.get_parameter_entry(name)? {
            Entry::List(nodes) => nodes
                .iter()
                .map(|node| -> Result<&T, Error> {
                    node.try_into()
                        .map_to_error_kind(ErrorKind::UnexpectedNodeType)
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
        &'a T: TryFrom<&'a Node, Error = DowncastNodeError>,
    {
        match self.get_parameter_entry(name)? {
            Entry::Value(node) => Ok(node
                .try_into()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            Entry::None => Ok(default
                .try_into()
                .map_to_error_kind(ErrorKind::UnexpectedNodeType)?),
            Entry::List(nodes) => Err(Error::too_many_parameters(nodes.len())),
        }
    }

    pub fn from_arguments(
        signature: &FunctionSignature,
        arguments: &[Node],
        options: Options,
    ) -> Result<ArgumentMapping, Error> {
        let mut parameters = VecDeque::from(arguments.to_vec());
        let mut mapping = ArgumentMapping::new();

        'outer: for argument in signature.arguments() {
            let mut matched = 0;
            let argument_options = argument.get_options();
            let name = argument.get_name();
            if parameters.is_empty() {
                match argument_options.cardinality() {
                    Cardinality::Required => {
                        return Err(Error::missing_parameter(name));
                    }
                    Cardinality::Optional => {
                        mapping.insert_none(name);
                        continue 'outer;
                    }
                    Cardinality::OptionalWithDefault { default } => {
                        mapping.insert_value(name, default);
                    }
                    Cardinality::Multiple { min, max } => {
                        if min > matched {
                            return Err(
                                Error::invalid_repeatable_parameter_count(
                                    name, min, max, matched,
                                ),
                            );
                        }
                    }
                }
            }

            if let Some(node) = parameters.front().cloned() {
                let node = match argument_options.preprocess() {
                    Preprocess::None => node,
                    Preprocess::Evaluate => node.evaluate(options)?,
                };

                match argument_options.cardinality() {
                    Cardinality::Required => {
                        if argument.has_node_type(node.node_type()) {
                            mapping.insert_value(name, node);
                            parameters.pop_front();
                            continue 'outer;
                        } else {
                            return Err(Error::invalid_parameter_type(
                                name,
                                argument_options.node_types(),
                                node.node_type(),
                            )
                            .with_optional_span(node.hull()));
                        }
                    }
                    Cardinality::Optional => {
                        if argument.has_node_type(node.node_type()) {
                            mapping.insert_value(name, node);
                            parameters.pop_front();
                            continue 'outer;
                        } else {
                            mapping.insert_none(name);
                            continue 'outer;
                        }
                    }
                    Cardinality::OptionalWithDefault { default } => {
                        if argument.has_node_type(node.node_type()) {
                            mapping.insert_value(name, node);
                            parameters.pop_front();
                            continue 'outer;
                        } else {
                            mapping.insert_value(name, default);
                            continue 'outer;
                        }
                    }
                    Cardinality::Multiple { min, max } => {
                        let mut values = Vec::new();

                        while let Some(node) = parameters.front().cloned() {
                            let node = match argument_options.preprocess() {
                                Preprocess::None => node,
                                Preprocess::Evaluate => {
                                    node.evaluate(options)?
                                }
                            };

                            if matched > max.unwrap_or(matched) {
                                return Err(
                                    Error::invalid_repeatable_parameter_count(
                                        name, min, max, matched,
                                    )
                                    .with_optional_span(node.hull()),
                                );
                            }
                            if !argument.has_node_type(node.node_type()) {
                                if matched >= min {
                                    mapping.insert_value_list(name, values);
                                    continue 'outer;
                                }
                                return Err(
                                    Error::invalid_repeatable_parameter_count(
                                        name, min, max, matched,
                                    )
                                    .with_optional_span(node.hull()),
                                );
                            } else {
                                if matched >= max.unwrap_or(usize::MAX) {
                                    return Err(
                                    Error::invalid_repeatable_parameter_count(
                                        name, min, max, matched,
                                    )
                                    .with_optional_span(node.hull()),
                                );
                                }
                                values.push(node);
                                parameters.pop_front();
                            }
                            matched += 1;
                        }
                        mapping.insert_value_list(name, values);
                        continue 'outer;
                    }
                }
            }
        }

        if !parameters.is_empty() {
            return Err(Error::too_many_parameters(parameters.len())
                .with_optional_span(
                    parameters.into_iter().collect::<Vec<_>>().combine_hulls(),
                ));
        }

        Ok(mapping)
    }
}
