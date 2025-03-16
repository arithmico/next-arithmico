use crate::core::{
    Context, Node, SerializeNode, SerializeNodeError, SerializeNodeUtils,
};

impl SerializeNodeUtils for Node {
    fn prepare_serialization(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.prepare_serialization(context),
            Node::Sum(sum) => sum.prepare_serialization(context),
            Node::Negate(negate) => negate.prepare_serialization(context),
            Node::Product(product) => product.prepare_serialization(context),
            Node::Division(division) => division.prepare_serialization(context),
            Node::Power(power) => power.prepare_serialization(context),
            Node::Tensor(tensor) => tensor.prepare_serialization(context),
            Node::Number(number) => number.prepare_serialization(context),
            Node::Symbol(symbol) => symbol.prepare_serialization(context),
            Node::Function(function) => function.prepare_serialization(context),
            Node::FunctionCall(function_call) => {
                function_call.prepare_serialization(context)
            }
            Node::And(and) => and.prepare_serialization(context),
            Node::Or(or) => or.prepare_serialization(context),
            Node::Equals(equals) => equals.prepare_serialization(context),
            Node::LessThan(less_than) => {
                less_than.prepare_serialization(context)
            }
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.prepare_serialization(context)
            }
            Node::GreaterThan(greater_than) => {
                greater_than.prepare_serialization(context)
            }
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.prepare_serialization(context)
            }
            Node::HostFunction(host_function) => {
                host_function.prepare_serialization(context)
            }
            Node::Definition(definition) => {
                definition.prepare_serialization(context)
            }
        }
    }
}

impl SerializeNode for Node {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.serialize(context),
            Node::Sum(sum) => sum.serialize(context),
            Node::Negate(negate) => negate.serialize(context),
            Node::Product(product) => product.serialize(context),
            Node::Division(division) => division.serialize(context),
            Node::Power(power) => power.serialize(context),
            Node::Tensor(tensor) => tensor.serialize(context),
            Node::Number(number) => number.serialize(context),
            Node::Symbol(symbol) => symbol.serialize(context),
            Node::Function(function) => function.serialize(context),
            Node::FunctionCall(function_call) => {
                function_call.serialize(context)
            }
            Node::And(and) => and.serialize(context),
            Node::Or(or) => or.serialize(context),
            Node::Equals(equals) => equals.serialize(context),
            Node::LessThan(less_than) => less_than.serialize(context),
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.serialize(context)
            }
            Node::GreaterThan(greater_than) => greater_than.serialize(context),
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.serialize(context)
            }
            Node::HostFunction(host_function) => {
                host_function.serialize(context)
            }
            Node::Definition(definition) => definition.serialize(context),
        }
    }
}
