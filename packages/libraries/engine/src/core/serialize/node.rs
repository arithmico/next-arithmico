use crate::Node;

use super::{
    SerializeNodeError, SerializeNodeOptions, serialize_node::SerializeNode,
    serialize_node_utils::SerializeNodeUtils,
};

mod and;
mod boolean;
mod definition;
mod division;
mod equals;
mod function;
mod function_call;
mod greater_than;
mod greater_than_or_equals;
mod host_function;
mod less_than;
mod less_than_or_equals;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

impl SerializeNodeUtils for Node {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.prepare_serialization(options),
            Node::Sum(sum) => sum.prepare_serialization(options),
            Node::Negate(negate) => negate.prepare_serialization(options),
            Node::Product(product) => product.prepare_serialization(options),
            Node::Division(division) => division.prepare_serialization(options),
            Node::Power(power) => power.prepare_serialization(options),
            Node::Tensor(tensor) => tensor.prepare_serialization(options),
            Node::Number(number) => number.prepare_serialization(options),
            Node::Symbol(symbol) => symbol.prepare_serialization(options),
            Node::Function(function) => function.prepare_serialization(options),
            Node::FunctionCall(function_call) => {
                function_call.prepare_serialization(options)
            }
            Node::And(and) => and.prepare_serialization(options),
            Node::Or(or) => or.prepare_serialization(options),
            Node::Equals(equals) => equals.prepare_serialization(options),
            Node::LessThan(less_than) => {
                less_than.prepare_serialization(options)
            }
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.prepare_serialization(options)
            }
            Node::GreaterThan(greater_than) => {
                greater_than.prepare_serialization(options)
            }
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.prepare_serialization(options)
            }
            Node::HostFunction(host_function) => {
                host_function.prepare_serialization(options)
            }
            Node::Definition(definition) => {
                definition.prepare_serialization(options)
            }
        }
    }
}

impl SerializeNode for Node {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.serialize(options),
            Node::Sum(sum) => sum.serialize(options),
            Node::Negate(negate) => negate.serialize(options),
            Node::Product(product) => product.serialize(options),
            Node::Division(division) => division.serialize(options),
            Node::Power(power) => power.serialize(options),
            Node::Tensor(tensor) => tensor.serialize(options),
            Node::Number(number) => number.serialize(options),
            Node::Symbol(symbol) => symbol.serialize(options),
            Node::Function(function) => function.serialize(options),
            Node::FunctionCall(function_call) => {
                function_call.serialize(options)
            }
            Node::And(and) => and.serialize(options),
            Node::Or(or) => or.serialize(options),
            Node::Equals(equals) => equals.serialize(options),
            Node::LessThan(less_than) => less_than.serialize(options),
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.serialize(options)
            }
            Node::GreaterThan(greater_than) => greater_than.serialize(options),
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.serialize(options)
            }
            Node::HostFunction(host_function) => {
                host_function.serialize(options)
            }
            Node::Definition(definition) => definition.serialize(options),
        }
    }
}
