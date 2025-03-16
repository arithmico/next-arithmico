use std::collections::VecDeque;

use crate::core::{
    argument_mapping::ArgumentMapping, Cardinality, EvaluateNodeContext,
    EvaluateNodeError, FunctionSignature, GetNodeType, Node, Preprocess,
};

use super::EvaluateNode;

pub fn map_function_parameters(
    signature: &FunctionSignature,
    parameters: &Vec<Node>,
    context: &EvaluateNodeContext,
) -> Result<ArgumentMapping, EvaluateNodeError> {
    let mut parameters = VecDeque::from(parameters.clone());
    let mut mapping = ArgumentMapping::new();
    'outer: for argument in signature.arguments() {
        let mut matched = 0;
        let options = argument.options();
        let name = argument.name();
        if parameters.is_empty() {
            let name = name.clone();
            match options.cardinality() {
                Cardinality::Required => {
                    return Err(EvaluateNodeError::missing_parameter(name));
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
                        return Err(EvaluateNodeError::invalid_repeatable_parameter_count(
                            name, min, max, matched));
                    }
                }
            }
        }
        while let Some(node) = parameters.front().cloned() {
            let name = name.clone();
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
                        return Err(EvaluateNodeError::invalid_parameter_type(
                            name,
                            options.node_types(),
                            node.node_type(),
                        )
                        .with_tracable(node));
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
                    if matched > max.unwrap_or(matched) {
                        return Err(EvaluateNodeError::invalid_repeatable_parameter_count(name, min, max, matched).with_tracable(node));
                    }
                    if argument.has_node_type(node.node_type()) {
                        mapping.insert_value(name, node);
                        parameters.pop_front();
                    } else {
                        if matched >= min {
                            continue 'outer;
                        }
                        return Err(EvaluateNodeError::invalid_repeatable_parameter_count(name, min, max, matched).with_tracable(node));
                    }
                    matched += 1;
                }
            }
        }
    }

    if !parameters.is_empty() {
        return Err(EvaluateNodeError::too_many_parameters(parameters.len())
            .with_tracable(parameters.into_iter().collect::<Vec<_>>()));
    }

    Ok(mapping)
}
