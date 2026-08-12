use std::collections::VecDeque;

use evaluator::Error;
use node::{Cardinality, FunctionSignature, GetNodeType, Node, Preprocess};
use trace::{CombineHulls, Tracable, TracableMut};

use crate::{ArgumentMapping, core::Context};

use super::EvaluateNode;

pub fn map_function_parameters(
    signature: &FunctionSignature,
    parameters: &Vec<Node>,
    context: &Context,
) -> Result<ArgumentMapping, Error> {
    let mut parameters = VecDeque::from(parameters.clone());
    let mut mapping = ArgumentMapping::new();
    'outer: for argument in signature.arguments() {
        let mut matched = 0;
        let options = argument.get_options();
        let name = argument.get_name();
        if parameters.is_empty() {
            let name = name;
            match options.cardinality() {
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
                        return Err(Error::invalid_repeatable_parameter_count(
                            name, min, max, matched,
                        ));
                    }
                }
            }
        }
        while let Some(node) = parameters.front().cloned() {
            let name = name;
            let node = match options.preprocess() {
                Preprocess::None => node,
                Preprocess::Evaluate => node.evaluate(context)?,
            };
            match options.cardinality() {
                Cardinality::Required => {
                    if argument.has_node_type(node.node_type()) {
                        mapping.insert_value(name, node);
                        parameters.pop_front();
                        continue 'outer;
                    } else {
                        return Err(Error::invalid_parameter_type(
                            name,
                            options.node_types(),
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
                        let node = match options.preprocess() {
                            Preprocess::None => node,
                            Preprocess::Evaluate => node.evaluate(context)?,
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
