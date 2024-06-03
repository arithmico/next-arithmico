use crate::core::{context::Context, node::SerializeNode};

use super::Node;

impl SerializeNode for Node {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        match self {
            Node::Number(node) => node.transform_before_serialization(context),
            Node::Symbol(node) => node.transform_before_serialization(context),
            Node::Boolean(node) => node.transform_before_serialization(context),
            Node::Negate(node) => node.transform_before_serialization(context),
            Node::Sum(node) => node.transform_before_serialization(context),
            Node::Product(node) => node.transform_before_serialization(context),
            Node::Division(node) => {
                node.transform_before_serialization(context)
            }
            Node::Power(node) => node.transform_before_serialization(context),
            Node::Tensor(node) => node.transform_before_serialization(context),
            Node::FunctionCall(node) => {
                node.transform_before_serialization(context)
            }
            Node::Function(node) => {
                node.transform_before_serialization(context)
            }
            Node::HostApiFunctionEndpoint(node) => {
                node.transform_before_serialization(context)
            }
            Node::Definition(node) => {
                node.transform_before_serialization(context)
            }
            Node::And(node) => node.transform_before_serialization(context),
            Node::Or(node) => node.transform_before_serialization(context),
            Node::Equals(node) => node.transform_before_serialization(context),
        }
    }

    fn serialize(&self, context: &Context) -> String {
        match self {
            Node::Number(node) => node.serialize(context),
            Node::Symbol(node) => node.serialize(context),
            Node::Boolean(node) => node.serialize(context),
            Node::Negate(node) => node.serialize(context),
            Node::Sum(node) => node.serialize(context),
            Node::Product(node) => node.serialize(context),
            Node::Division(node) => node.serialize(context),
            Node::Power(node) => node.serialize(context),
            Node::Tensor(node) => node.serialize(context),
            Node::FunctionCall(node) => node.serialize(context),
            Node::Function(node) => node.serialize(context),
            Node::HostApiFunctionEndpoint(node) => node.serialize(context),
            Node::Definition(node) => node.serialize(context),
            Node::And(node) => node.serialize(context),
            Node::Or(node) => node.serialize(context),
            Node::Equals(node) => node.serialize(context),
        }
    }
}
